let currentQuestion = null;
let currentQuestionNumber = 0;

const totalQuestions = 10;
let score = 0;

const question = document.getElementById("question");
const questionText = document.getElementById("question-text");
const options = document.getElementById("options");
const feedback = document.getElementById("feedback");
const nextButton = document.getElementById("next");
const progress = document.getElementById("progress");

async function loadQuestion() {
    currentQuestionNumber++;

    feedback.textContent = "";
    nextButton.hidden = true;
    options.innerHTML = "";

    progress.textContent =
        `${currentQuestionNumber} / ${totalQuestions}`;

    try {
        const response = await fetch("/api/practice/question");

        if (!response.ok) {
            throw new Error("Failed to fetch question");
        }

        currentQuestion = await response.json();

        displayQuestion();
    } catch (error) {
        console.error(error);
        feedback.textContent = "Failed to load question.";
    }
}

function displayQuestion() {
    if (currentQuestion.question_type === "hiragana_to_romaji") {
        question.textContent = currentQuestion.character;
        questionText.textContent = "What is the romaji?";
    } else {
        question.textContent = currentQuestion.romaji;
        questionText.textContent = "Which hiragana is this?";
    }

    currentQuestion.option.forEach((answer) => {
        const button = document.createElement("button");

        button.textContent = answer;

        button.addEventListener("click", () => {
            checkAnswer(answer);
        });

        options.appendChild(button);
    });
}

function checkAnswer(answer) {
    const isCorrect = answer === currentQuestion.correct_answer;

    if (isCorrect) {
        score++;
        feedback.textContent = "Correct!";
    } else {
        feedback.textContent =
            `Incorrect. The correct answer is ${currentQuestion.correct_answer}.`;
    }

    const buttons = options.querySelectorAll("button");

    buttons.forEach((button) => {
        button.disabled = true;
    });

    nextButton.hidden = false;

    if (currentQuestionNumber === totalQuestions) {
        nextButton.textContent = "Finish";
    }
}

nextButton.addEventListener("click", () => {
    if (currentQuestionNumber === totalQuestions) {
        showResult();
        return;
    }

    loadQuestion();
});

function showResult() {
    const accuracy = Math.round((score / totalQuestions) * 100);

    question.textContent = "Practice Complete";
    question.classList.add("result-title");

    questionText.textContent = `Score: ${score} / ${totalQuestions}`;

    options.innerHTML = "";

    feedback.textContent = `Accuracy: ${accuracy}%`;

    progress.textContent = "";

    nextButton.textContent = "Practice Again";
    nextButton.hidden = false;

    nextButton.onclick = restartPractice;
}

function restartPractice() {
    currentQuestionNumber = 0;
    score = 0;

    question.classList.remove("result-title");

    nextButton.onclick = null;
    nextButton.textContent = "Next";

    loadQuestion();
}

loadQuestion();