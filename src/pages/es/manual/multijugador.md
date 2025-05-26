---
title: "Multijugador"
layout: "../../../layouts/MarkdownLayout.astro"
---

# Multijugador

    Nota: Esta parte se encuentra en alpha. Es probable encontrar errores. Si se encuentra alguno, cerrar y abrir el programa y volver a abrir la sala arreglará el problema

     Nota 2: Esta parte no funciona en MacOS, ya que no permite desactivar el firewall del dispositivo, lo cuál es necesario para el correcto funcionamiento del programa.

Para jugar al modo multijugador ir a `Multijugador` > `Red local`. Aparecerá el siguiente menú:
![Menú unirse salas](/kekris/manual/multiplayer/roomsempty.png)  
Aquí aparecerán todas las salas abierta en la red local.  
Si estas en Windows al entrar en este menú preguntará si permites a este programa pasar a través del firewall. Permite el acceso en redes privadas. También es posible en redes públicas, pero no es recomendado, ya que dejarías un puerto abierto expuesto para que te ataquen el dispositivo. Si sigues teniendo problemas, desactiva el firewall temporalmente.
En Linux será necesario desactivarlo desde la línea de comandos con:

```
sudo systemctl stop firewalld (mejor que disable ya que volverá a arrancarse en inicio)
```

ó

```
sudo ufw disable (no volverá a arrancarse en inicio)
```

Escoger el que tenga instalado el sistema.

## Alojar partida

Para alojar la partida simplemente hay que darle al botón `Alojar partida`. Se creará la sala y podrá unirse todo el mundo que está en la red local.
Verás el siguiente menú:  
![Host room](/kekris/manual/multiplayer/hostroom.png)  
Al no haber todavía jugadores unidos no será posible iniciar la partida y el botón de `Jugar` estará deshabilitado.

## Unirse a partida

Si estás en el menú de listado de salas y hay una partida siendo alojada verás una entrada en el listado.  
![One room in list](/kekris/manual/multiplayer/oneroomlist.png)  
Podrás ver el nombre de la sala, cuánta gente hay conectada y cuántas partidas se han jugado. Si pinchas encima de la sala te unirás a ella. Se podrá rechazar la conexión si hay un error interno o si la sala ya está llena. Por ahora el límite son 16 jugadores (incluyendo al anfitrión de la partida) y no hay posibilidad para cambiarla. No hay sistema para hacer las partidas privadas o expulsar a una persona en concreto. También es posible que entre alguien con nombres que puedan ser ofensivos.

Si la conexión es aceptada se verá lo siguiente:
![One room in list](/kekris/manual/multiplayer/clientroom.png)

## Jugar

Cuándo haya más de un jugador en la sala el anfitrión podrá escoger empezar la partida en cualquier momento. Se llevará a los jugadores ante un tablero dónde empezará la partida:
![Partida multijugador](/kekris/manual/multiplayer/multiplayergame.png)  
Este tablero varía del de un solo jugador en que a los lados se podrán ver las partidas de otros jugadores (en este caso solo uno) y abajo se podrán ver las diferentes estrategias de ataque que hay (en el caso de ser solo dos jugadores son irrelevantes). Una vez terminada la partida se volverá al menú de la sala
