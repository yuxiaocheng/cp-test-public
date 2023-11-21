FROM cp-controller-cn-beijing.cr.volces.com/cp-v2-system/java:v2.0.0

COPY . .
RUN mkdir -p /tmp/test123
RUN java --version
RUN mvn --version