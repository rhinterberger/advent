package com.aoc2015.day3;

import java.nio.file.Files;
import java.nio.file.Path;
import java.util.HashSet;
import java.util.Objects;

public class Main {

    public static void main(String[] args) throws Exception{
        String directions = Files.readString(Path.of("input.txt"));

        part1(directions);
        part2(directions);
    }

    private static void part1(String directions) {
        HashSet<House> housesVisited = new HashSet<>();
        House santasHouse = new House(0,0);

        for(int direction : directions.codePoints().toArray()) {
            housesVisited.add(santasHouse);
            santasHouse = move(santasHouse, direction);
        }

        System.out.println( housesVisited.size() );
    }

    private static void part2(String directions) {
        HashSet<House> housesVisited = new HashSet<>();

        House santasHouse = new House(0,0);
        House botsHouse = new House(0,0);
        housesVisited.add(santasHouse);

        int[] dir = directions.codePoints().toArray();
        for(int pos=0; pos < dir.length;) {

            santasHouse = move(santasHouse, dir[pos++]);
            housesVisited.add(santasHouse);

            botsHouse = move(botsHouse, dir[pos++]);
            housesVisited.add(botsHouse);
        }

        System.out.println( housesVisited.size() );
    }

    private static House move(House currentHouse, int direction) {
        House nextHouse = new House(currentHouse.x, currentHouse.y);
        switch(direction) {
            case '^' -> nextHouse.y--;
            case 'v' -> nextHouse.y++;
            case '<' -> nextHouse.x--;
            case '>' -> nextHouse.x++;
        }
        return nextHouse;
    }

    private static class House {
        int x;
        int y;

        public House(int x, int y) {
            this.x = x;
            this.y = y;
        }

        @Override
        public boolean equals(Object o) {
            if (this == o)
                return true;
            if (o == null)
                return false;
            if (getClass() != o.getClass())
                return false;
            House point = (House) o;
            return Objects.equals(x, point.x)
                && Objects.equals(y, point.y);
        }

        @Override
        public int hashCode() {
            return Objects.hash(x,y);
        }
    }
}