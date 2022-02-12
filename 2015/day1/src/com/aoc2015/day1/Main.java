package com.aoc2015.day1;

import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.PrimitiveIterator;

public class Main {

    public static void main(String[] args) throws Exception {
        String data = getInput();

        int floor = getFloor(data);
        System.out.println(floor);

        int basement = getFirstBasementStop(data);
        System.out.println(basement);
    }

    private static String getInput() throws Exception{
        return new String(Files.readAllBytes(Paths.get("input.txt")));
    }

    private static int getFloor(String data) {
        return data
                .codePoints()
                .reduce(0, (currentFloor, direction) ->
                                   currentFloor + moveElevator(direction)
                );
    }

    private static int moveElevator(int direction) {
        return switch(direction) {
            case '(' ->  1;
            case ')' -> -1;
            default  ->  0;
        };
    }

    private static int getFirstBasementStop(String data) throws Exception {
        int floor=0;
        int position=0;

        PrimitiveIterator.OfInt chars = data.codePoints().iterator();
        while(floor != -1 && chars.hasNext()) {
            position++;
            floor += moveElevator(chars.next());
        }
        if(floor == -1)
            return position;

        throw new Exception("Basement not reached");
    }
}