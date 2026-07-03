// A runnable Java example: build a frame through the binding.
//
//   cargo build -p wickra-xray-c
//   mvn -f bindings/java/pom.xml -q package -DskipTests
//   javac -cp bindings/java/target/classes examples/java/Frame.java -d examples/java/out
//   java --enable-native-access=ALL-UNNAMED \
//        -Dnative.lib.dir=target/debug \
//        -cp "bindings/java/target/classes;examples/java/out" Frame
import org.wickra.xray.Xray;

public final class Frame {
    private static final String SPEC =
            "{\"dataset_ref\":\"m\",\"symbol\":\"AAA\",\"panels\":["
                    + "{\"kind\":\"footprint\",\"price_bin\":1.0,\"bucket_ms\":60000}]}";

    private static final String LOAD =
            "{\"cmd\":\"load\",\"dataset\":{\"trades\":["
                    + "{\"ts\":1000,\"price\":100.4,\"qty\":2.0,\"side\":\"buy\"},"
                    + "{\"ts\":1400,\"price\":101.8,\"qty\":0.5,\"side\":\"sell\"}]}}";

    public static void main(String[] args) {
        try (Xray xray = new Xray(SPEC)) {
            xray.command(LOAD);
            String response = xray.command("{\"cmd\":\"frame\"}");
            System.out.println("wickra-xray " + Xray.version());
            System.out.println(response);
        }
    }
}
