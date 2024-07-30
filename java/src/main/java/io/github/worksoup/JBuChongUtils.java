package io.github.worksoup;

import org.jetbrains.annotations.Contract;

public final class JBuChongUtils {
    public static Class forName(String name) {
        Class c;
        try {
            c = Class.forName(name);
        } catch (ClassNotFoundException e) {
            throw new RuntimeException(e);
        }
        return c;
    }

    @Contract("null, _ -> false")
    public static boolean isInstanceOf(Object obj, String className) {
        return JBuChongUtils.forName(className).isInstance(obj);
    }


    public static String primitiveByteArrayToString(byte[] bytes) {
        StringBuilder hexArray = new StringBuilder();
        if (bytes != null) {
            for (int num : bytes) {
                hexArray.append(String.format("%02x", num & 0xFF));
            }
        }
        return hexArray.toString();
    }

}