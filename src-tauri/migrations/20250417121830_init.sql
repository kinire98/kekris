CREATE TABLE games(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    game_type TEXT,
    id_game INTEGER
);
CREATE TABLE game_info (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    piece_moves         INTEGER NOT NULL, 
    spins               INTEGER NOT NULL, 
    lines_cleared       INTEGER NOT NULL, 
    pieces_used         INTEGER NOT NULL, 
    singles             INTEGER NOT NULL, 
    doubles             INTEGER NOT NULL, 
    triples             INTEGER NOT NULL, 
    tetrises            INTEGER NOT NULL, 
    tspins              INTEGER NOT NULL, 
    tspin_singles       INTEGER NOT NULL, 
    tspin_doubles       INTEGER NOT NULL, 
    tspin_triples       INTEGER NOT NULL, 
    minitspins          INTEGER NOT NULL, 
    minitspin_singles   INTEGER NOT NULL
);

CREATE TABLE classic (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    time_endured INTEGER NOT NULL,
    points INTEGER NOT NULL,
    level_reached INTEGER NOT NULL,
    game_info_id INTEGER NOT NULL,
    FOREIGN KEY (game_info_id) REFERENCES game_info(id) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE lines (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    time_endured INTEGER NOT NULL,
    game_info_id INTEGER NOT NULL,
    FOREIGN KEY (game_info_id) REFERENCES game_info(id) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE blitz (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    points INTEGER NOT NULL,
    game_info_id INTEGER NOT NULL,
    FOREIGN KEY (game_info_id) REFERENCES game_info(id) ON UPDATE CASCADE ON DELETE CASCADE
);

