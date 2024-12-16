import boardScreenshot from './screenshots/board.png';
import cardScreenshot from './screenshots/card.png';
import WindMoveCardScreenshot from './screenshots/wind-move-card.png';
import ShadowBoardScreenshot from './screenshots/way-of-the-shadow-board.png';
import LightBoardScreenshot from './screenshots/way-of-the-light-board.png';

export const rulesData = [
  {
    path: '/rules/base-game',
    label: 'Base Game',
    content: `
# Onitama

Onitama is an elegant and simple game which captures the essence of martial arts.

Each game is quick, usually 15 minutes in length.

## Board

![Onitama Board](${boardScreenshot})

Each game will use 5 cards chosen randomly from the 16 game cards.

The board begins with each player controlling **4 pawns** and **1 king**.

Each player has a hand of **2 cards**.

One card is a spare on the side of the board (or on mobile highlighted in the hand).

## Playing the Game

Players alternate turns until the game has ended.

On a player's turn, they select one of their two cards, determining where they can move one of their pieces.

![Ox Card](${cardScreenshot})

For example, the Ox card lets the player move a piece either:

- One square forward
- One square right
- One square behind.

**Note: Card moves are relative to the player's perspective, so the card will rotate 180 degrees when the opponent has it.**

Pieces can be moved to any square that is not occupied by a same-colored piece.

The played card then switches with the spare card.

### Capture

If a piece is moved to the same square as an opponent's piece, the opponent's piece is **captured** and removed from the game.

### Discarding

If you have no moves that you can legally make, you instead will discard a card, playing it and not moving a piece.

If you can play a move, you cannot discard.

## Winning the Game

The game can be won in one of two ways:

### King Capture

If a player **captures** an opponent's **king**, they win the game.

### Base Capture

If a player moves their **king** to the **starting square** of the opponent (the colored square), they win the game.
    `,
  },
  {
    path: '/rules/way-of-the-wind',
    label: 'Way of the Wind',
    content: `
# Way of the Wind

In the Way of the Wind expansion, a neutral **Wind Spirit** piece and new special move cards are introduced.

## Setup

The board and player pieces are set up the same as in the base game.

In the center square, there is a new neutral piece called the **Wind Spirit**.

<div style="text-align: center; margin: 16px 0;">
  [WindSpiritIcon size="large"]
</div>

The game uses 5 cards as normal but can also include special Wind Spirit move cards.

![Wind Spirit Move Card Example](${WindMoveCardScreenshot})

## Playing the Game

Play proceeds as in the normal game, with the following changes:

- When using a regular move card, you may use it to move one of your pieces or the **Wind Spirit**[WindSpiritIcon size="small"]
- If you use a Wind Spirit move card, you first move one of your other pieces according to the top grid, then you **must** move the Wind Spirit (if able) using one of the moves on the bottom grid.

![Wind Spirit Move Card Example](${WindMoveCardScreenshot})

## Moving the Wind Spirit

The **Wind Spirit** moves like any other piece when using a move card.

- If the Wind Spirit lands on a Pawn (friendly or enemy), it does not capture it. Instead, they **swap places**.
- The Wind Spirit cannot move into the same space as a King.
- No piece can move onto the Wind Spirit, effectively blocking its square.

## Alternate Setup

By default, 25% of games will include the Wind Spirit, and the number of Wind Spirit move cards is determined randomly with a weighted distribution.

You can adjust the setup for the Wind Spirit game using the settings menu:

- This allows you to force the Wind Spirit into every game.
- You can also specify the exact number of Wind Spirit move cards in the game.
- If you do not want to play with the Wind Spirit at all, you can simply disable the Way of the Wind card set.

    `,
  },
  {
    path: '/rules/way-of-the-shadow',
    label: 'Way of the Shadow',
    content: `
# Way of the Shadow

The **Light and Shadow** expansion introduces a new piece: the **Ninja**, capable of secret movements and surprise attacks.

<div style="text-align: center; margin: 16px 0;">
  [NinjaIcon size="large"]
</div>

In the *Way of the Shadow* each player deploys their own Ninja.

## Setup

![Way of the Shadow Board](${ShadowBoardScreenshot})

The board and player pieces are set up as follows:

- Each player uses **2 Pawns**, which are placed in the far corners of their side of the board, with their **King** in their **Base**.  
- The **Ninja** is placed *randomly* in one of the spaces adjacent to the **King**, and begins the game hidden.

<div style="text-align: center; margin: 16px 0;">
  [NinjaIcon size="large" hidden]
</div>

## Playing the Game

Play proceeds similarly to the base game, but with the following changes:

### Movement

- After using a move card with another piece, players may move their **Ninja** [NinjaIcon size="small"] using the same move card.
- Moving the **Ninja** is optional, but if you have a move card that can be used by another piece, you must move that piece first.
- Ninjas remain hidden [NinjaIcon size="small" hidden] unless revealed [NinjaIcon size="small"], and become hidden again at the start of your next turn.
- Two hidden Ninjas can occupy the same space.

### Capturing a Ninja with Your Other Pieces

- If one of your Pawns or your King moves onto an opponent's Ninja, whether hidden [NinjaIcon size="small" color="blue" hidden] or revealed [NinjaIcon size="small" color="blue"], it is captured.

### Capturing with Your Ninja

- To capture a Pawn or a revealed Ninja, move your Ninja to a space occupied by that piece. This will reveal your Ninja.
- To capture your opponent's hidden Ninja, first reveal your Ninja by clicking on it, then move it to the square where you think your opponent's Ninja is. If the Ninja is there, it will be captured. Your Ninja will remain revealed until your next turn whether their Ninja was there or not.

## Alternate Setup

By default, 5% of games will be either Light or Shadow games, with Shadow games being much more common.

You can adjust this in the settings.

- You can force every game to use the Light and Shadow Expansion.
- You can make every game either a *Way of the Shadow* or *Way of the Light* game.
- If you do not want to play with the Light and Shadow expansion, you can also simply disable it.
  `,
  },
  {
    path: '/rules/way-of-the-light',
    label: 'Way of the Light',
    content: `
# Way of the Light

The **Light and Shadow** expansion introduces a new piece: the **Ninja**, capable of secret movements and surprise attacks.

<div style="text-align: center; margin: 16px 0;">
  [NinjaIcon size="large" color="blue"]
</div>

The *Way of the Light* introduces asymmetrical gameplay, where one player controls two **Ninjas**, while the other uses **4 Pawns** and a **King** as in the base game.

## Setup

![Way of the Light Board](${LightBoardScreenshot})

- The Ninja player starts with **2 Ninjas**, placed randomly in spaces on their side of the board.
- The King player begins with **4 Pawns** and **1 King**, arranged as in the base game.
- The King player always takes the first turn.

## Playing the Game

Play proceeds as in the normal game, with the following changes:

### Ninja Player

- The Ninja player moves their Ninjas using the move cards in the same way as Pawns.
- If a hidden Ninja [NinjaIcon size="small" color="blue" hidden] captures a Pawn, it becomes revealed [NinjaIcon size="small" color="blue"] until the Ninja player's next turn.

### Winning the Game

The game ends when one player fulfills a victory condition:

1. The Ninja player has no remaining pieces.
2. The Ninja player captures the opponent's **King**.
3. The **King** reaches the Ninja player's **Base**.

## Alternate Setup

By default, 5% of games will be either Light or Shadow games, with Light games being much less common.

You can adjust this in the settings:

- Force every game to use the Light and Shadow Expansion.
- Choose to make every game either *Way of the Light* or *Way of the Shadow*.
- Disable the Light and Shadow Expansion entirely if you prefer not to use it.
  `,
  },
];

export default rulesData;
