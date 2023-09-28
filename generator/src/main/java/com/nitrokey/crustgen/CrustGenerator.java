/*
 * Copyright 2018 OpenAPI-Generator Contributors (https://openapi-generator.tech)
 * Copyright 2018 SmartBear Software
 * Copyright 2023 Nitrokey
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
package com.nitrokey.crustgen;

import static org.openapitools.codegen.utils.StringUtils.camelize;

import java.io.File;
import java.io.IOException;
import java.io.Writer;
import java.math.BigDecimal;
import java.math.BigInteger;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.EnumSet;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Locale;
import java.util.Map;
import java.util.Objects;
import java.util.Optional;

import org.openapitools.codegen.CliOption;
import org.openapitools.codegen.CodegenConfig;
import org.openapitools.codegen.CodegenConstants;
import org.openapitools.codegen.CodegenDiscriminator;
import org.openapitools.codegen.CodegenModel;
import org.openapitools.codegen.CodegenOperation;
import org.openapitools.codegen.CodegenProperty;
import org.openapitools.codegen.CodegenType;
import org.openapitools.codegen.SupportingFile;
import org.openapitools.codegen.languages.AbstractRustCodegen;
import org.openapitools.codegen.meta.features.ClientModificationFeature;
import org.openapitools.codegen.meta.features.DocumentationFeature;
import org.openapitools.codegen.meta.features.GlobalFeature;
import org.openapitools.codegen.meta.features.ParameterFeature;
import org.openapitools.codegen.meta.features.SchemaSupportFeature;
import org.openapitools.codegen.meta.features.SecurityFeature;
import org.openapitools.codegen.meta.features.WireFormatFeature;
import org.openapitools.codegen.model.ModelMap;
import org.openapitools.codegen.model.ModelsMap;
import org.openapitools.codegen.model.OperationMap;
import org.openapitools.codegen.model.OperationsMap;
import org.openapitools.codegen.utils.ModelUtils;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import com.samskivert.mustache.Mustache;
import com.samskivert.mustache.Template;

import io.swagger.v3.oas.models.Operation;
import io.swagger.v3.oas.models.media.ArraySchema;
import io.swagger.v3.oas.models.media.Schema;
import io.swagger.v3.oas.models.media.StringSchema;
import io.swagger.v3.parser.util.SchemaTypeUtil;
import joptsimple.internal.Strings;

public class CrustGenerator extends AbstractRustCodegen implements CodegenConfig {
  private final Logger LOGGER = LoggerFactory.getLogger(CrustGenerator.class);

  private boolean useSingleRequestParameter = false;
  private boolean supportAsync = true;
  private boolean supportMiddleware = false;
  private boolean supportMultipleResponses = false;
  private boolean preferUnsignedInt = false;
  private boolean bestFitInt = false;

  public static final String PACKAGE_NAME = "packageName";
  public static final String PACKAGE_VERSION = "packageVersion";
  public static final String REQWEST_LIBRARY = "reqwest";
  public static final String SUPPORT_ASYNC = "supportAsync";
  public static final String SUPPORT_MIDDLEWARE = "supportMiddleware";
  public static final String SUPPORT_MULTIPLE_RESPONSES = "supportMultipleResponses";
  public static final String PREFER_UNSIGNED_INT = "preferUnsignedInt";
  public static final String BEST_FIT_INT = "bestFitInt";

  protected String packageName = "openapi";
  protected String packageVersion = "1.0.0";
  protected String apiDocPath = "docs/";
  protected String modelDocPath = "docs/";
  protected String apiFolder = "src/apis";
  protected String modelFolder = "src/models";

  static final String PRODUCE_MULTIPLE_MEDIA_TYPE = "x-produceMultipleMediaTypes";
  static final String CONSUME_MULTIPLE_MEDIA_TYPE = "x-consumeMultipleMediaTypes";
  static final String MEDIA_TYPE = "mediaType";
  static final String MEDIA_IS_JSON = "mediaIsJson";
  static final String MIME_NO_CONTENT = "MimeNoContent";

  public CodegenType getTag() {
    return CodegenType.CLIENT;
  }

  public String getName() {
    return "crust";
  }

  public String getHelp() {
    return "Generates a rust client library customized for Nitrokey.";
  }

  public CrustGenerator() {
    super();

    modifyFeatureSet(features -> features
        .includeDocumentationFeatures(DocumentationFeature.Readme)
        .wireFormatFeatures(EnumSet.of(WireFormatFeature.JSON, WireFormatFeature.XML, WireFormatFeature.Custom))
        .securityFeatures(EnumSet.of(
            SecurityFeature.BasicAuth,
            SecurityFeature.BearerToken,
            SecurityFeature.ApiKey,
            SecurityFeature.OAuth2_Implicit))
        .excludeGlobalFeatures(
            GlobalFeature.XMLStructureDefinitions,
            GlobalFeature.Callbacks,
            GlobalFeature.LinkObjects,
            GlobalFeature.ParameterStyling)
        .excludeSchemaSupportFeatures(
            SchemaSupportFeature.Polymorphism)
        .excludeParameterFeatures(
            ParameterFeature.Cookie)
        .includeClientModificationFeatures(
            ClientModificationFeature.BasePath,
            ClientModificationFeature.UserAgent));

    outputFolder = "generated-code/rust";

    modelTemplateFiles.put("model.mustache", ".rs");

    modelDocTemplateFiles.put("model_doc.mustache", ".md");
    apiDocTemplateFiles.put("api_doc.mustache", ".md");

    // default HIDE_GENERATION_TIMESTAMP to true
    hideGenerationTimestamp = Boolean.TRUE;

    embeddedTemplateDir = templateDir = "crust";

    defaultIncludes = new HashSet<>(
        Arrays.asList(
            "map",
            "array"));

    languageSpecificPrimitives = new HashSet<>(
        Arrays.asList(
            "i8", "i16", "i32", "i64",
            "u8", "u16", "u32", "u64",
            "f32", "f64", "isize", "usize",
            "char", "bool", "str", "String"));

    instantiationTypes.clear();
    /*
     * instantiationTypes.put("array", "GoArray");
     * instantiationTypes.put("map", "GoMap");
     */

    typeMapping.clear();
    typeMapping.put("integer", "i32");
    typeMapping.put("long", "i64");
    typeMapping.put("number", "f32");
    typeMapping.put("float", "f32");
    typeMapping.put("double", "f64");
    typeMapping.put("boolean", "bool");
    typeMapping.put("string", "String");
    typeMapping.put("UUID", "String");
    typeMapping.put("URI", "String");
    typeMapping.put("date", "String");
    typeMapping.put("DateTime", "String");
    typeMapping.put("password", "String");

    // most versatile way to represent binary data
    typeMapping.put("file", "std::vec::Vec<u8>");
    typeMapping.put("binary", "std::vec::Vec<u8>");
    typeMapping.put("ByteArray", "String");
    typeMapping.put("object", "serde_json::Value");
    typeMapping.put("AnyType", "serde_json::Value");

    // no need for rust
    // importMapping = new HashMap<String, String>();

    cliOptions.clear();
    cliOptions.add(new CliOption(CodegenConstants.PACKAGE_NAME, "Rust package name (convention: lowercase).")
        .defaultValue("openapi"));
    cliOptions.add(new CliOption(CodegenConstants.PACKAGE_VERSION, "Rust package version.")
        .defaultValue("1.0.0"));
    cliOptions.add(new CliOption(CodegenConstants.HIDE_GENERATION_TIMESTAMP,
        CodegenConstants.HIDE_GENERATION_TIMESTAMP_DESC)
        .defaultValue(Boolean.TRUE.toString()));
    cliOptions.add(new CliOption(CodegenConstants.USE_SINGLE_REQUEST_PARAMETER,
        CodegenConstants.USE_SINGLE_REQUEST_PARAMETER_DESC, SchemaTypeUtil.BOOLEAN_TYPE)
        .defaultValue(Boolean.FALSE.toString()));
    cliOptions.add(new CliOption(SUPPORT_ASYNC,
        "If set, generate async function call instead. This option is for 'reqwest' library only",
        SchemaTypeUtil.BOOLEAN_TYPE)
        .defaultValue(Boolean.TRUE.toString()));
    cliOptions.add(new CliOption(SUPPORT_MIDDLEWARE,
        "If set, add support for reqwest-middleware. This option is for 'reqwest' library only",
        SchemaTypeUtil.BOOLEAN_TYPE)
        .defaultValue(Boolean.FALSE.toString()));
    cliOptions.add(new CliOption(SUPPORT_MULTIPLE_RESPONSES,
        "If set, return type wraps an enum of all possible 2xx schemas. This option is for 'reqwest' library only",
        SchemaTypeUtil.BOOLEAN_TYPE)
        .defaultValue(Boolean.FALSE.toString()));
    cliOptions.add(new CliOption(CodegenConstants.ENUM_NAME_SUFFIX, CodegenConstants.ENUM_NAME_SUFFIX_DESC)
        .defaultValue(this.enumSuffix));
    cliOptions.add(new CliOption(PREFER_UNSIGNED_INT, "Prefer unsigned integers where minimum value is >= 0",
        SchemaTypeUtil.BOOLEAN_TYPE)
        .defaultValue(Boolean.FALSE.toString()));
    cliOptions.add(new CliOption(BEST_FIT_INT, "Use best fitting integer type where minimum or maximum is set",
        SchemaTypeUtil.BOOLEAN_TYPE)
        .defaultValue(Boolean.FALSE.toString()));

    supportedLibraries.put(REQWEST_LIBRARY, "HTTP client: Reqwest.");

    CliOption libraryOption = new CliOption(CodegenConstants.LIBRARY, "library template (sub-template) to use.");
    libraryOption.setEnum(supportedLibraries);
    // set reqwest as the default
    libraryOption.setDefault(REQWEST_LIBRARY);
    cliOptions.add(libraryOption);
    setLibrary(REQWEST_LIBRARY);
  }

  @Override
  public ModelsMap postProcessModels(ModelsMap objs) {
    // process enum in models
    return postProcessModelsEnum(objs);
  }

  @SuppressWarnings("static-method")
  public Map<String, ModelsMap> postProcessAllModels(Map<String, ModelsMap> objs) {
    // Index all CodegenModels by model name.
    Map<String, CodegenModel> allModels = new HashMap<>();

    for (Map.Entry<String, ModelsMap> entry : objs.entrySet()) {
      String modelName = toModelName(entry.getKey());
      List<ModelMap> models = entry.getValue().getModels();
      for (ModelMap mo : models) {
        CodegenModel cm = mo.getModel();
        allModels.put(modelName, cm);
      }
    }

    for (Map.Entry<String, ModelsMap> entry : objs.entrySet()) {
      List<ModelMap> models = entry.getValue().getModels();
      for (ModelMap mo : models) {
        CodegenModel cm = mo.getModel();
        if (cm.discriminator != null) {
          List<Object> discriminatorVars = new ArrayList<>();
          for (CodegenDiscriminator.MappedModel mappedModel : cm.discriminator.getMappedModels()) {
            CodegenModel model = allModels.get(mappedModel.getModelName());
            Map<String, Object> mas = new HashMap<>();
            mas.put("modelName", camelize(mappedModel.getModelName()));
            mas.put("mappingName", mappedModel.getMappingName());

            // TODO: deleting the variable from the array was
            // problematic; I don't know what this is supposed to do
            // so I'm just cloning it for the moment
            List<CodegenProperty> vars = new ArrayList<>(model.getVars());
            vars.removeIf(p -> p.name.equals(cm.discriminator.getPropertyName()));
            mas.put("vars", vars);
            discriminatorVars.add(mas);
          }
          // TODO: figure out how to properly have the original property type that didn't
          // go through toVarName
          String vendorExtensionTagName = cm.discriminator.getPropertyName();
          cm.vendorExtensions.put("x-tag-name", vendorExtensionTagName);
          cm.vendorExtensions.put("x-mapped-models", discriminatorVars);
        }
      }
    }
    return objs;
  }

  @Override
  public void addOperationToGroup(String tag, String resourcePath, Operation operation, CodegenOperation co,
      Map<String, List<CodegenOperation>> operations) {
    super.addOperationToGroup(tag, resourcePath, operation, co, operations);
    processProducesConsumes(co);
  }

  private void processProducesConsumes(CodegenOperation op) {

    if (op.hasConsumes) {

      if (op.consumes.size() > 1) {
        op.vendorExtensions.put(CONSUME_MULTIPLE_MEDIA_TYPE, "true");
      }
      for (Map<String, String> m : op.consumes) {
        processMediaType(op, m);

      }

    }
    if (op.hasProduces) {

      LOGGER.info("op.produces: {}", op.produces.size());

      if (op.produces.size() > 1) {
        op.vendorExtensions.put(PRODUCE_MULTIPLE_MEDIA_TYPE, "true");
      }

      for (Map<String, String> m : op.produces) {
        processMediaType(op, m);
      }
    }
  }

  private void processMediaType(CodegenOperation op, Map<String, String> m) {
    String mediaType = m.get(MEDIA_TYPE);

    if (org.apache.commons.lang3.StringUtils.isBlank(mediaType))
      return;

    // String mimeType = getMimeDataType(mediaType);

    LOGGER.warn("mediaType: {}", mediaType);

    if (isJsonMimeType(mediaType)) {

      LOGGER.warn("mediaType: {} is json", mediaType);
      m.put(MEDIA_IS_JSON, "true");
    }

  }

  public String escapeIdentifier(String prefix, String name) {
    if (org.apache.commons.lang3.StringUtils.isBlank(prefix))
      return name;

    if (isReservedWord(name)) {
      name = prefix + name;
    }
    if (name.matches("^\\d.*")) {
      name = prefix + name; // e.g. 200Response => Model200Response (after camelize)
    }
    if (languageSpecificPrimitives.contains(name)) {
      name = prefix + name;
    }
    if (typeMapping.containsValue(name)) {
      name = prefix + name;
    }
    return name;
  }

  public String toTypeName(String prefix, String name) {
    name = escapeIdentifier(prefix, camelize(sanitizeName(name)));
    return name;
  }

  /**
   * Escapes a reserved word as defined in the `reservedWords` array. Handle
   * escaping
   * those terms here. This logic is only called if a variable matches the
   * reserved words
   *
   * @return the escaped term
   */
  @Override
  public String escapeReservedWord(String name) {
    return "_" + name; // add an underscore to the name
  }

  /**
   * override with any special text escaping logic to handle unsafe
   * characters so as to avoid code injection
   *
   * @param input String to be cleaned up
   * @return string with unsafe characters removed or escaped
   */
  @Override
  public String escapeUnsafeCharacters(String input) {
    // TODO: check that this logic is safe to escape unsafe characters to avoid code
    // injection
    return input;
  }

  /**
   * Escape single and/or double quote to avoid code injection
   *
   * @param input String to be cleaned up
   * @return string with quotation mark removed or escaped
   */
  public String escapeQuotationMark(String input) {
    // TODO: check that this logic is safe to escape quotation mark to avoid code
    // injection
    return input.replace("\"", "\\\"");
  }

  @Override
  public void processOpts() {
    super.processOpts();

    if (additionalProperties.containsKey(CodegenConstants.ENUM_NAME_SUFFIX)) {
      enumSuffix = additionalProperties.get(CodegenConstants.ENUM_NAME_SUFFIX).toString();
    }

    if (additionalProperties.containsKey(CodegenConstants.PACKAGE_NAME)) {
      setPackageName((String) additionalProperties.get(CodegenConstants.PACKAGE_NAME));
    } else {
      setPackageName("openapi");
    }

    // If no version is provided in additional properties, version from API
    // specification is used.
    // If none of them is provided then fallback to default version
    if (additionalProperties.containsKey(CodegenConstants.PACKAGE_VERSION)) {
      setPackageVersion((String) additionalProperties.get(CodegenConstants.PACKAGE_VERSION));
    } else if (openAPI != null && openAPI.getInfo() != null && openAPI.getInfo().getVersion() != null) {
      setPackageVersion(openAPI.getInfo().getVersion());
    }

    if (additionalProperties.containsKey(CodegenConstants.USE_SINGLE_REQUEST_PARAMETER)) {
      this.setUseSingleRequestParameter(convertPropertyToBoolean(CodegenConstants.USE_SINGLE_REQUEST_PARAMETER));
    }
    writePropertyBack(CodegenConstants.USE_SINGLE_REQUEST_PARAMETER, getUseSingleRequestParameter());

    if (additionalProperties.containsKey(SUPPORT_ASYNC)) {
      this.setSupportAsync(convertPropertyToBoolean(SUPPORT_ASYNC));
    }
    writePropertyBack(SUPPORT_ASYNC, getSupportAsync());

    if (additionalProperties.containsKey(SUPPORT_MIDDLEWARE)) {
      this.setSupportMiddleware(convertPropertyToBoolean(SUPPORT_MIDDLEWARE));
    }
    writePropertyBack(SUPPORT_MIDDLEWARE, getSupportMiddleware());

    if (additionalProperties.containsKey(SUPPORT_MULTIPLE_RESPONSES)) {
      this.setSupportMultipleReturns(convertPropertyToBoolean(SUPPORT_MULTIPLE_RESPONSES));
    }
    writePropertyBack(SUPPORT_MULTIPLE_RESPONSES, getSupportMultipleReturns());

    if (additionalProperties.containsKey(PREFER_UNSIGNED_INT)) {
      this.setPreferUnsignedInt(convertPropertyToBoolean(PREFER_UNSIGNED_INT));
    }
    writePropertyBack(PREFER_UNSIGNED_INT, getPreferUnsignedInt());

    if (additionalProperties.containsKey(BEST_FIT_INT)) {
      this.setBestFitInt(convertPropertyToBoolean(BEST_FIT_INT));
    }
    writePropertyBack(BEST_FIT_INT, getBestFitInt());

    additionalProperties.put(CodegenConstants.PACKAGE_NAME, packageName);
    additionalProperties.put(CodegenConstants.PACKAGE_VERSION, packageVersion);

    additionalProperties.put("apiDocPath", apiDocPath);
    additionalProperties.put("modelDocPath", modelDocPath);

    if (REQWEST_LIBRARY.equals(getLibrary())) {
      additionalProperties.put(REQWEST_LIBRARY, "true");
    } else {
      LOGGER.error("Unknown library option (-l/--library): {}", getLibrary());
    }

    apiTemplateFiles.put(getLibrary() + "/api.mustache", ".rs");

    modelPackage = packageName;
    apiPackage = packageName;

    supportingFiles.add(new SupportingFile("README.mustache", "", "README.md"));
    supportingFiles.add(new SupportingFile("gitignore.mustache", "", ".gitignore"));
    supportingFiles.add(new SupportingFile("model_mod.mustache", modelFolder, "mod.rs"));
    supportingFiles.add(new SupportingFile("lib.mustache", "src", "lib.rs"));
    supportingFiles.add(new SupportingFile("Cargo.mustache", "", "Cargo.toml"));

    supportingFiles.add(new SupportingFile(getLibrary() + "/api_mod.mustache", apiFolder, "mod.rs"));
    supportingFiles
        .add(new SupportingFile(getLibrary() + "/configuration.mustache", apiFolder, "configuration.rs"));

    // add lambda for sanitize version (e.g. v1.2.3-beta => 1.2.3-beta)
    additionalProperties.put("lambdaVersion", new Mustache.Lambda() {
      @Override
      public void execute(Template.Fragment fragment, Writer writer) throws IOException {
        String content = fragment.execute();
        // remove v or V
        content = content.trim().replace("v", "");
        content = content.replace("V", "");
        writer.write(content);
      }
    });

  }

  private boolean getSupportAsync() {
    return supportAsync;
  }

  private void setSupportAsync(boolean supportAsync) {
    this.supportAsync = supportAsync;
  }

  private boolean getSupportMiddleware() {
    return supportMiddleware;
  }

  private void setSupportMiddleware(boolean supportMiddleware) {
    this.supportMiddleware = supportMiddleware;
  }

  public boolean getSupportMultipleReturns() {
    return supportMultipleResponses;
  }

  public void setSupportMultipleReturns(boolean supportMultipleResponses) {
    this.supportMultipleResponses = supportMultipleResponses;
  }

  public boolean getPreferUnsignedInt() {
    return preferUnsignedInt;
  }

  public void setPreferUnsignedInt(boolean preferUnsignedInt) {
    this.preferUnsignedInt = preferUnsignedInt;
  }

  public boolean getBestFitInt() {
    return bestFitInt;
  }

  public void setBestFitInt(boolean bestFitInt) {
    this.bestFitInt = bestFitInt;
  }

  private boolean getUseSingleRequestParameter() {
    return useSingleRequestParameter;
  }

  private void setUseSingleRequestParameter(boolean useSingleRequestParameter) {
    this.useSingleRequestParameter = useSingleRequestParameter;
  }

  @Override
  public String apiFileFolder() {
    return (outputFolder + File.separator + apiFolder).replace("/", File.separator);
  }

  @Override
  public String modelFileFolder() {
    return (outputFolder + File.separator + modelFolder).replace("/", File.separator);
  }

  @Override
  public String apiDocFileFolder() {
    return (outputFolder + "/" + apiDocPath).replace('/', File.separatorChar);
  }

  @Override
  public String modelDocFileFolder() {
    return (outputFolder + "/" + modelDocPath).replace('/', File.separatorChar);
  }

  @Override
  public String getTypeDeclaration(Schema p) {

    Schema unaliasSchema = unaliasSchema(p);
    if (ModelUtils.isArraySchema(unaliasSchema)) {
      ArraySchema ap = (ArraySchema) unaliasSchema;
      Schema inner = ap.getItems();
      if (inner == null) {
        LOGGER.warn("{}(array property) does not have a proper inner type defined.Default to string",
            ap.getName());
        inner = new StringSchema().description("TODO default missing array inner type to string");
      }
      return "Vec<" + getTypeDeclaration(inner) + ">";
    } else if (ModelUtils.isMapSchema(unaliasSchema)) {
      Schema inner = ModelUtils.getAdditionalProperties(openAPI, unaliasSchema);
      if (inner == null) {
        LOGGER.warn("{}(map property) does not have a proper inner type defined. Default to string",
            unaliasSchema.getName());
        inner = new StringSchema().description("TODO default missing map inner type to string");
      }
      return "::std::collections::HashMap<String, " + getTypeDeclaration(inner) + ">";
    }

    // Not using the supertype invocation, because we want to UpperCamelize
    // the type.
    String schemaType = getSchemaType(unaliasSchema);
    if (typeMapping.containsKey(schemaType)) {
      return typeMapping.get(schemaType);
    }

    if (typeMapping.containsValue(schemaType)) {
      return schemaType;
    }

    if (languageSpecificPrimitives.contains(schemaType)) {
      return schemaType;
    }

    // return fully-qualified model name
    // crate::models::{{classnameFile}}::{{classname}}

    return "crate::models::" + toModelName(schemaType);
  }

  @Override
  public String getSchemaType(Schema p) {
    String schemaType = super.getSchemaType(p);
    String type = typeMapping.getOrDefault(schemaType, schemaType);

    // Implement integer type fitting (when property is enabled)
    if (Objects.equals(p.getType(), "integer")) {
      boolean bestFit = convertPropertyToBoolean(BEST_FIT_INT);
      boolean preferUnsigned = convertPropertyToBoolean(PREFER_UNSIGNED_INT);

      BigInteger minimum = Optional.ofNullable(p.getMinimum()).map(BigDecimal::toBigInteger).orElse(null);
      boolean exclusiveMinimum = Optional.ofNullable(p.getExclusiveMinimum()).orElse(false);

      boolean unsigned = preferUnsigned && canFitIntoUnsigned(minimum, exclusiveMinimum);

      if (Strings.isNullOrEmpty(p.getFormat())) {
        if (bestFit) {
          return bestFittingIntegerType(
              minimum,
              exclusiveMinimum,
              Optional.ofNullable(p.getMaximum()).map(BigDecimal::toBigInteger).orElse(null),
              Optional.ofNullable(p.getExclusiveMaximum()).orElse(false),
              preferUnsigned);
        } else {
          return unsigned ? "u32" : "i32";
        }
      } else {
        switch (p.getFormat()) {
          case "int32":
            return unsigned ? "u32" : "i32";
          case "int64":
            return unsigned ? "u64" : "i64";
        }
      }
    }
    return type;
  }

  @Override
  public void postProcessModelProperty(CodegenModel model, CodegenProperty property) {
    super.postProcessModelProperty(model, property);

    // If a property is both nullable and non-required then we represent this using
    // a double Option
    // which requires the `serde_with` extension crate for deserialization.
    // See:
    // https://docs.rs/serde_with/latest/serde_with/rust/double_option/index.html
    if (property.isNullable && !property.required) {
      additionalProperties.put("serdeWith", true);
    }

  }

  @Override
  public OperationsMap postProcessOperationsWithModels(OperationsMap objs, List<ModelMap> allModels) {
    OperationMap objectMap = objs.getOperations();
    List<CodegenOperation> operations = objectMap.getOperation();
    for (CodegenOperation operation : operations) {
      // http method verb conversion, depending on client library (e.g. Hyper: PUT =>
      // Put, Reqwest: PUT => put)
      if (REQWEST_LIBRARY.equals(getLibrary())) {
        operation.httpMethod = operation.httpMethod.toUpperCase(Locale.ROOT);
      }

      // add support for single request parameter using x-group-parameters
      if (!operation.vendorExtensions.containsKey("x-group-parameters") && useSingleRequestParameter) {
        operation.vendorExtensions.put("x-group-parameters", Boolean.TRUE);
      }

    }
    return objs;
  }

  public void setPackageName(String packageName) {
    this.packageName = packageName;
  }

  public void setPackageVersion(String packageVersion) {
    this.packageVersion = packageVersion;
  }

  @Override
  public String toDefaultValue(Schema p) {
    if (p.getDefault() != null) {
      return p.getDefault().toString();
    } else {
      return null;
    }
  }

}
