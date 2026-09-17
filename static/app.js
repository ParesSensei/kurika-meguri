let hiragana = [];
let currentIndex = 0;

async function loadHiragana() {
    try{
        const response = await fetch("http://localhost:3000/api/hiragana");
        hiragana = await response.json();

        displayHiragana();
    } catch (err) {
        console.error("failed to load hiragana: ", err)
    }

}

function displayHiragana() {
    const current = hiragana[currentIndex];

    document.getElementById("character").textContent = current.character;
    document.getElementById("romaji").textContent = current.romaji;
    document.getElementById("progress").textContent =
        `${currentIndex + 1} / ${hiragana.length}`;
}

document.getElementById("next").addEventListener("click", () => {
    if (currentIndex < hiragana.length -1) {
        currentIndex++;
        displayHiragana();
    }
})

document.getElementById("previous").addEventListener("click", () => {
    if (currentIndex > 0) {
        currentIndex--;
        displayHiragana();
    }
})

loadHiragana();