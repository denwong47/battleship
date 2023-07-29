# Battleship
### Single Player game through a self hosted API End point

![CI status](https://github.com/denwong47/battleship/actions/workflows/CI.yml/badge.svg?branch=main)
![Github Pages](https://github.com/denwong47/battleship/actions/workflows/gh_pages.yml/badge.svg?branch=main)
![Publish status](https://github.com/denwong47/battleship/actions/workflows/publish.yml/badge.svg?branch=main)

This simple API End point hosts a single-player variation of the "Battleship"
game, in which a player call "shots" at the host's fleet of warships randomly
distributed on a concealed gridded board, with the objective of destroying all
of them using the least number of strikes.

![Hasbro version of the game](https://m.media-amazon.com/images/I/71GcJTrP0HL._AC_SL1500_.jpg)

In contrast to the traditional board game, this API only has one board held by
the host. The host does not strike back at the player.

## What's the point of this?

This was written to help a friend of mine understand ETL processes involving API
requests, hopefully in a somewhat meaningful and purposeful context. For this
reason the host includes "silly" features such as simulated network failures
(e.g. timeout).

The game itself serves no purpose and is not necessarily entertaining. The true
purpose is to algorithmically solve the game via the API hooks, which records
statistics that cannot be easily cheated.

## How to use the API End point

1. Find the [latest release](https://github.com/denwong47/battleship/releases), and download the binary for your platform.

2. *Optional*: Setup `host.json`. A [sample](https://github.com/denwong47/battleship/blob/main/host.json.sample) is provided.

3. Run the binary with any override parameters:

    - `--addr <hostname>` - only bind the host to this address. Can be a hostname or an IP. Defaults to `0.0.0.0` which binds to addresses of all interfaces.
    - `--port <port>` - an integer from `0` to `65535` for the port to be hosted on.

4. You should see some log statements like below, your host is good to go!

```
2023-07-30T21:01:33.866779 │ INFO      │ Starting `battleship` server.
2023-07-30T21:01:33.866932 │ DEBUG     │ Initialising `tide::Server`...
2023-07-30T21:01:33.867156 │ DEBUG     │ `tide::Server` created...
2023-07-30T21:01:33.867174 │ INFO      │ Ready to listen at localhost:8080...
```

## Available Paths

### `/new`

#### Syntax:
```
/new?width=<u64>&height=<u64>&ship_count=<usize>
```

*Create a new game with the stated parameters.*

All parameters are optional:

- `width`: board width, default `16`,
- `height`: board height, default `16`, and
- `ship_count`: number of ships to aim to create. There is no guarantee that
this amount of ships will be present in the resultant board - this was simply
the aimed amount. Each ship creation is tried for `32` times before giving up.
The number of ships will never exceed this number.

#### Example return:

```json
{
  "action": "createGame",
  "game": {
    "active": true,
    "elapsed": 0.000089,
    "ship_intel": [
      {
        "damages": 0,
        "ship_type": "Battleship",
        "status": "Undiscovered",
        "uuid": "593f093a-620b-4bf4-ba31-e8ee5b4d9de5"
      },
      {
        "damages": 0,
        "ship_type": "Cruiser",
        "status": "Undiscovered",
        "uuid": "01e96180-2910-4f09-930d-f75bafcacf53"
      },
      {
        "damages": 0,
        "ship_type": "AircraftCarrier",
        "status": "Undiscovered",
        "uuid": "919c5d58-aabf-481e-ae7f-3f601816f629"
      },
      {
        "damages": 0,
        "ship_type": "Battleship",
        "status": "Undiscovered",
        "uuid": "4d22ea2a-5ec3-4117-a8e5-ed6ecae8efcf"
      }
    ],
    "size": [
      8,
      6
    ],
    "strikes": 0,
    "uuid": "9e3bfde9-4248-49fd-8550-3e773c3deca6"
  },
  "success": true
}
```

Example board

        0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5
     0     ██
     1     ██            ░░
     2 ██  ██        ██  ██    ██  ░░
     3 ██            ██  ██    ██
     4   ██      ╳╳      ██    ██
     5   ██    ░░╳╳            ██    ╳╳
     6   ██    ██                    ╳╳
     7   ██    ██    ░░              ╳╳
     8         ██                    ░░
     9     ██████                    ██
    10                 ╳╳░░      ██  ╳╳
    11 ██              ╳╳░░      ██  ╳╳
    12 ██      ░░      ██            ░░
    13 ██        ░░        ╳╳████
    14
    15         ██████████


### `/status/<uuid>`

#### Syntax:

```
/status/<uuid>
```

*Give the status of the board without giving away any secrets of the game.*

#### Example return:

```json
{
  "active": true,
  "elapsed": 991.084999,
  "ship_intel": [
    {
      "damages": 0,
      "ship_type": "Battleship",
      "status": "Undiscovered",
      "uuid": "593f093a-620b-4bf4-ba31-e8ee5b4d9de5"
    },
    {
      "damages": 0,
      "ship_type": "Cruiser",
      "status": "Undiscovered",
      "uuid": "01e96180-2910-4f09-930d-f75bafcacf53"
    },
    {
      "damages": 0,
      "ship_type": "AircraftCarrier",
      "status": "Undiscovered",
      "uuid": "919c5d58-aabf-481e-ae7f-3f601816f629"
    },
    {
      "damages": 0,
      "ship_type": "Battleship",
      "status": "Undiscovered",
      "uuid": "4d22ea2a-5ec3-4117-a8e5-ed6ecae8efcf"
    }
  ],
  "size": [
    8,
    6
  ],
  "strikes": 0,
  "uuid": "9e3bfde9-4248-49fd-8550-3e773c3deca6"
}
```

### `/strike/<uuid>`

#### Syntax:

```
/strike/<uuid>?x=<u64>&y=<u64>
```

*Perform a strike on the specified coordinates.*

#### Example return:

- Missed:

  ```json
  {
    "coordinates": {
      "x": 12,
      "y": 4
    },
    "hit": false,
    "intel": null,
    "ships_remaining": 7,
    "uuid": "281834dc-a688-45a2-ba72-311a757cc0ca"
  }
  ```
- Hit:

  ```json
  {
    "coordinates": {
      "x": 7,
      "y": 9
    },
    "hit": true,
    "intel": {
      "damages": 1,
      "ship_type": "Battleship",
      "status": "Discovered",
      "uuid": "91efacf4-f97d-4478-b1aa-a7df6d2534f7"
    },
    "ships_remaining": 7,
    "uuid": "af7d69e9-1809-4d1e-97aa-d76ce3fa303e"
  }
  ```
- Coordinates out of bounds:

  ```json
  {
    "error": "CoordinatesOutOfBounds",
    "message": "Coordinates (7, 16) is out of bounds."
  }
  ```

### `/drop/<uuid>`

#### Syntax:

```
/drop/<uuid>
```

*Finish a game, and return the current unobscured state of the board.*

This will remove the game from the server permanently.

#### Example return:

```json
{
  "frozen": false,
  "position_index": {
    ...
  },
  "ships": [
    {
      "coordinates": {
        "x": 1,
        "y": 2
      },
      "orientation": "Down",
      "ship_type": "Cruiser",
      "strikes": [
        null,
        null,
        null
      ],
      "uuid": "c79d876c-2778-4aca-ac6d-f643b14e2b14"
    },
    {
      "coordinates": {
        "x": 0,
        "y": 1
      },
      "orientation": "Right",
      "ship_type": "Submarine",
      "strikes": [
        null,
        null,
        null
      ],
      "uuid": "150024d8-53c3-4b58-9e13-d11380b47885"
    },
    {
      "coordinates": {
        "x": 2,
        "y": 2
      },
      "orientation": "Down",
      "ship_type": "Cruiser",
      "strikes": [
        null,
        null,
        null
      ],
      "uuid": "f34f6cd3-e9f7-4b4a-b448-eee830941fc4"
    }
  ],
  "size": [
    6,
    6
  ],
  "start_time": "2023-07-31T08:20:51.814026",
  "strikes": [],
  "uuid": "255f3c2b-b5dd-4850-b0e9-bfc6551747b4"
}
```

### `/info`

#### Syntax:
```
/info
```

*Prints out information about the state of the app.*

#### Example return:

```json
{
  "page_visits": {
    "/drop/55a8e295-909c-4054-a18f-87ef6002e343": 1,
    "/info": 2,
    "/new": 2,
    "/status/55a8e295-909c-4054-a18f-87ef6002e343": 1,
    "/status/v": 1,
    "/strike/55a8e295-909c-4054-a18f-87ef6002e343": 6
  },
  "start_time": "2023-08-01T21:09:31.404040"
}
```

If the executable is built with the `debug` feature, a full visit log will also
be included.
