FROM maven:3-eclipse-temurin-11 AS build

WORKDIR /build

COPY . .

RUN mvn package

FROM openapitools/openapi-generator-cli

COPY --from=build /build/target/crust-openapi-generator-1.0.0.jar /generators/crust.jar

COPY docker-entrypoint.sh /usr/local/bin/
RUN ln -s /usr/local/bin/docker-entrypoint.sh /usr/local/bin/openapi-generator

ENTRYPOINT ["/usr/local/bin/docker-entrypoint.sh"]