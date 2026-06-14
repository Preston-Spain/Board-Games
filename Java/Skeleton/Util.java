package Skeleton;

import java.util.Random;
import java.util.Scanner;

public class Util {

    Random r = new Random();
    Scanner input = new Scanner(System.in);

    public int roll(int max) {
        return r.nextInt(max);
    }

    public int roll() {
        return r.nextInt(6);
    }

    // public boolean 

    public class Character {
        String Name;
        int x;
        int y;
    }
}
