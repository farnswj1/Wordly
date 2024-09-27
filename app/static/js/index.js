const NUMBER_OF_GUESSES = 6;
let guessesRemaining = NUMBER_OF_GUESSES;
let currentGuess = [];
let nextLetter = 0;
const ws = new WebSocket("/ws");

const initBoard = () => {
  const game = document.getElementById("game");
  const board = document.createElement("div");
  board.id = "game-board";
  board.className = "d-flex flex-column align-items-center mb-3";

  // Set up game board
  for (let i = 0; i < NUMBER_OF_GUESSES; i++) {
    const row = document.createElement("div");
    row.className = "letter-row";

    for (let j = 0; j < 5; j++) {
      const box = document.createElement("div");
      box.className = "letter-box";
      row.appendChild(box);
    }

    board.appendChild(row);
  }

  const keyboard = document.createElement("div");
  keyboard.id = "keyboard";

  // Set up keyboard
  const keyboardRows = ["first", "second", "third"];
  const keys = [
    ["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P"],
    ["A", "S", "D", "F", "G", "H", "J", "K", "L"],
    ["Del", "Z", "X", "C", "V", "B", "N", "M", "Enter"]
  ];

  for (let i = 0; i < 3; i++) {
    const rowNode = document.createElement("div");
    rowNode.className = `${keyboardRows[i]}-row d-flex flex-row justify-content-center my-1`;

    for (const char of keys[i]) {
      const button = document.createElement("button");
      button.className = "keyboard-button";
      button.innerHTML = char;
      rowNode.appendChild(button);
    }

    keyboard.appendChild(rowNode);
  }

  keyboard.addEventListener("click", (e) => {
    const target = e.target;
    if (!target.classList.contains("keyboard-button")) {
      return;
    }

    let key = target.textContent;
    if (key === "Del") {
      key = "Backspace";
    }

    document.dispatchEvent(new KeyboardEvent("keyup", { key: key }));
  });

  game.appendChild(board);
  game.appendChild(keyboard);
};

ws.addEventListener("open", initBoard);
ws.addEventListener("message", (event) => {
  const [type, message] = event.data.split(':');

  if (type == 'invalid') {
    toastr.error(message);
    return;
  }
  else if (guessesRemaining <= 0 && type == 'final') {
    toastr.error("You've run out of guesses! Game over!");
    toastr.error(`The word was: "${message}"`);
    return;
  }

  const row = document.getElementsByClassName("letter-row")[6 - guessesRemaining];
  const letterColor = Array(5).fill("lightgray");

  // Check for matches
  for (let i = 0; i < 5; i++) {
    switch (message[i]) {
    case '-':  // no match
      break;
    case '*':  // correct letter in wrong position
      letterColor[i] = "yellow";
      break;
    default:  // correct letter and position
      letterColor[i] = "green";
    }
  }

  for (let i = 0; i < 5; i++) {
    const box = row.children[i];
    const delay = 250 * i;

    setTimeout(() => {
      //flip box
      animateCSS(box, "flipInX");
      //shade box
      box.style.backgroundColor = letterColor[i];
      shadeKeyBoard(box.innerHTML, letterColor[i]);
    }, delay);
  }

  if (/[a-z]{5}/gi.test(message)) {
    toastr.success("You guessed right! Game over!");
    guessesRemaining = 0;
    return;
  } else {
    guessesRemaining -= 1;
    currentGuess = [];
    nextLetter = 0;
  }
});

const shadeKeyBoard = (letter, color) => {
  for (const elem of document.getElementsByClassName("keyboard-button")) {
    if (elem.textContent === letter) {
      let oldColor = elem.style.backgroundColor;
      if (oldColor === "green") {
        return;
      }

      if (oldColor === "yellow" && color !== "green") {
        return;
      }

      elem.style.backgroundColor = color;
      break;
    }
  }
}

const deleteLetter = () => {
  let row = document.getElementsByClassName("letter-row")[6 - guessesRemaining];
  let box = row.children[nextLetter - 1];
  box.textContent = "";
  box.classList.remove("filled-box");
  currentGuess.pop();
  nextLetter -= 1;
}

const checkGuess = () => {
  const guessString = currentGuess.join("");

  if (guessString.length != 5) {
    toastr.error("Not enough letters!");
    return;
  }

  ws.send(guessString);
}

const insertLetter = (pressedKey) => {
  if (nextLetter >= 5) {
    return;
  }

  pressedKey = pressedKey.toLowerCase();
  let row = document.getElementsByClassName("letter-row")[6 - guessesRemaining];
  let box = row.children[nextLetter];
  animateCSS(box, "pulse");
  box.textContent = pressedKey;
  box.classList.add("filled-box");
  currentGuess.push(pressedKey);
  nextLetter += 1;
}

const animateCSS = (element, animation, prefix = "animate__") => (
  // We create a Promise and return it
  new Promise((resolve, reject) => {
    const animationName = `${prefix}${animation}`;
    const baseAnimationName = `${prefix}animated`;
    element.style.setProperty("--animate-duration", "0.3s");
    element.classList.add(baseAnimationName, animationName);

    // When the animation ends, we clean the classes and resolve the Promise
    const handleAnimationEnd = (event) => {
      event.stopPropagation();
      element.classList.remove(baseAnimationName, animationName);
      resolve("Animation ended");
    };

    element.addEventListener("animationend", handleAnimationEnd, { once: true });
  }
));

document.addEventListener("keyup", (e) => {
  if (guessesRemaining === 0) {
    return;
  }

  let pressedKey = String(e.key);
  if (pressedKey === "Backspace" && nextLetter > 0) {
    deleteLetter();
    return;
  }

  if (pressedKey === "Enter") {
    checkGuess();
    return;
  }

  let found = pressedKey.match(/[a-z]/gi);
  if (!found || found.length > 1) {
    return;
  }

  insertLetter(pressedKey);
});
