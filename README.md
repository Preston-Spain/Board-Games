# Board-Games

* Subsystems
# Basic game systems
* Roll Dice, and store latest rolls
* Card system
* Property system for monopoly

# Application
* Get squares to show up
* Get pngs or another image file to work


# Monopoly

##
* []
* []
* []
* []
## start up
* [] How many characters are human
* [] 4 characters
* [] give everyone 1500 dollars
* [] render board
* [] place players
* [] Each player rolls die to see who runs first
## loop (A)
### loop (B)
* [] Player rolls for distance (log die number)
* [] Check if player rolled doubles thrice, if so go to jail immedately
* [] Check if passes go, if so add 200 dollors
* [] Move player to new tile
* [] Check for tile commands. Buyable, rent payment, Special cards/ jail, Auctionable
* [] If the player choses to land on a tile that they do not own and attempt to auction it or do not have sufficent funds loop C begins.
### loop C
* [] Check if player has enough money, if not they are forcefully abstained
* [] If player slections auction, player places a bid, or abstains
* [] If the player abstains all future bids in this auction event will skip this player
* [] 
* [] Return to first player in que, or next player in que, otherwise until one player is left

* [] Check if player rolled doubles, if so loop again from B [End loop B]
* [] Check if player wishes to trade with another player
* [] player makes offer, and demands
* [] opposing player accepts or declines
* [] Go to next player or restart order if is last, looping from A [End loop A]