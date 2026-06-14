package Skeleton;

import Skeleton.Monopoly.Command;
import java.util.TreeSet;

public class Monopoly {
    // classes
    public class Effect {
        Command command;
        String commandWord;
        int commandNum;
    }

    public class Card {
        String name;
        String def;
        Effect effect;
    }

    public class Tile {
        String Tile;
        String def;
        int cost;
        int level;
        Effect eff;
        PIECE ownership;
    }

    public class GamePeice {
        int money;
        int snakeEyesWatch;
        int[] lastRoll = new int[2];
        boolean alive;
        Character c;
        PIECE piece;
        TreeSet<Card> inventory;
    }

    //Enum
    enum PIECE {
        RED,
        BLUE,
        GREEN,
        WHITE
    }

    enum Command {
        BUY,
        JAIL,
        MOVE,
        MONEY,
        NONE
    }

    // functions
    public boolean prompt(String title, String def) {
        System.out.println(title);
        System.out.println(def);
        System.out.println("(Y/N)");

        return false;
    }
}
