package com.aoc2015.day2;

import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;

public class Main {

    public static void main(String[] args) throws Exception {
        List<String> lines = Files.readAllLines(Path.of("input.txt"));
        List<int[]> dimensions = lines
                .stream()
                .map(Main::parseDimensions)
                .toList();

        int area = dimensions
                .stream()
                .map(Main::getArea)
                .reduce(0, Integer::sum);
        System.out.println(area);

        int ribbon = dimensions
                .stream()
                .map(Main::getRibbon)
                .reduce(0, Integer::sum);
        System.out.println(ribbon);
    }

    private static int[] parseDimensions(String line) {
        String[] dimensions = line.split("x");

        int l = Integer.parseInt(dimensions[0]);
        int w = Integer.parseInt(dimensions[1]);
        int h = Integer.parseInt(dimensions[2]);

        return new int[]{l, w, h};
    }

    private static int getArea(int[] dimensions) {

        int[] surfaces = getSurfaces(dimensions);
        int smallestSide = getSmallestSide(surfaces);

        return 2 * (surfaces[0] + surfaces[1] + surfaces[2] ) + smallestSide;
    }

    private static int[] getSurfaces(int[] dimensions) {
        int l = dimensions[0];
        int w = dimensions[1];
        int h = dimensions[2];

        int[] surfaces = new int[3];
        surfaces[0] = l * w;
        surfaces[1] = w * h;
        surfaces[2] = h * l;
        return surfaces;
    }

    private static int getSmallestSide(int[] surfaces) {
        int min = Integer.MAX_VALUE;
        for(int surface : surfaces) {
            min = Math.min(surface, min);
        }
        return min;
    }

    private static int getRibbon(int[] dimensions) {

        int wrap = getWrap(dimensions);
        int bow = getBow(dimensions);

        return wrap + bow;
    }

    private static int getWrap(int[] sides) {

        int longest = 0;
        for(int side : sides) {
            longest = Math.max(longest, side);
        }

        return 2* (sides[0] + sides[1] + sides[2] - longest);
    }

    private static int getBow(int[] dimensions) {
        return dimensions[0]*dimensions[1]*dimensions[2];
    }
}