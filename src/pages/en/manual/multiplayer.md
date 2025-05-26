---
title: "Multiplayer"
layout: "../../../layouts/MarkdownLayout.astro"
---

# Multiplayer

    Note: This part of the program is in alpha version. It's very likely that you will encounter some errors. If that's the case, close and open the program and the room again.

     Note 2: This part doesn't work in MacOS because it doesn't allow to disable the device's firewall and that's neccesary for the correct behaviour of the program.

To play multiplayer mode go to `Multiplayer` > `Local Network`. It will appear the following menu:
![Menú unirse salas](/kekris/manual/multiplayer/roomsempty.png)  
Here will appear all rooms open in the LAN.
If you are on Windows, at entering this menu it will ask if you allow this program to pass through the firewall. Allow access from private networks. It is also possible in public networks but it is not recommended because you will leave open an exposed port. That's is an entry point for an attack to the device. If there is problems deactivate it temporarilly.
If you are on Linux you will have to deactivate it from the command line:

```
sudo systemctl stop firewalld (better than disable because it will start again at boot)
```

ó

```
sudo ufw disable (it won't start at boot)
```

Choose the one that your system has.

## Host room

Para alojar la partida simplemente hay que darle al botón `Alojar partida`. Se creará la sala y podrá unirse todo el mundo que está en la red local.
To host a room you simply have to press the button `Host room`. The room will be created and anyone on the local network can join.
You will see the following menu:  
![Host room](/kekris/manual/multiplayer/hostroom.png)  
There is no players yet so it is not possible to restart the room and the `Play` button is disabled.

## Join game

If you are on the room listing and there is a game being hosted you will see an entry in the listing.
![One room in list](/kekris/manual/multiplayer/oneroomlist.png)  
The room's name will be shown with the amount of people connected and how many games have been played. If you click on the room you will join. The connection can be refused on the basis of an internal error or the room being full. For now the limit is 16 players (including the game's host) and there is no posibility to change it yet. There is no system to make the games private o kick out an specific person. It is also possible that someone enters the room with an offensive name.

If the connection is accepted you will see the following
![One room in list](/kekris/manual/multiplayer/clientroom.png)

## Jugar

When there is more than one player the host can choose to start the game at any given time. Players will be taken to a board where the game will start:  
![Partida multijugador](/kekris/manual/multiplayer/multiplayergame.png)  
This board only changes from the single player one in that you can see the other players games (in this case only one) and below it you can see the different attack strategies (in the case of two players is irrelevant). When the game finishes, players will be taken back to the room menu.
