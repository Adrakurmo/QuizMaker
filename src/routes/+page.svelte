<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
    import { slide } from "svelte/transition";

  type QuizItem = {
    id: number;
    name: string;
  }

  type QuizQuestion = {
    text: string;
    answers: QuestionAnswer[];
  }

  type QuestionAnswer = {
    text: string;
    is_correct: boolean;
  }
  
  let isDarkMode = $state(window.matchMedia('(prefers-color-scheme: dark)').matches);
  let currentView = $state("add");
  let quizzes: QuizItem[] = $state([]);
  let quizQuestions: QuizQuestion[] = $state([]);
  let quizName: string = $state("GRUG");
 
  $effect(() => {
    if (isDarkMode) {
      document.body.classList.add('dark-mode');
    } else {
      document.body.classList.remove('dark-mode');
    }
  });

  function toogleTheme() {
    isDarkMode = !isDarkMode;
  }

  async function newQuiz(event: Event) {
    currentView = "add";
    event.preventDefault();
    // quizzes.push({
    //   id: counter,
    //   name: "GRUG" + counter
    // })
  }

  async function allQuizzes(event: Event) {
    event.preventDefault();
    currentView = "list";
    quizQuestions = [];
  }

  async function addQuestion(event:Event) {
    event.preventDefault();
    quizQuestions.push({
      text: "",
      // is_multiple_choice: false,
      answers: [
        {
          text: "",
          is_correct: false,
        },
        {
          text: "",
          is_correct: false,
        },
        {
          text: "",
          is_correct: false,
        },
        {
          text: "",
          is_correct: false,
        }
      ],
    })
  }

  async function quit(event: Event) {
    event.preventDefault();
    console.log("Quit Clicked");
  }

  async function removeAnswer(event: Event, question: QuizQuestion, answer: QuestionAnswer) {
    event.preventDefault();
    const idx = question.answers.indexOf(answer);
    if (idx != -1) question.answers.splice(idx, 1);
  }

  async function removeQuestion(event: Event, question: QuizQuestion) {
    event.preventDefault();
    const idx = quizQuestions.indexOf(question);
    if (idx != -1) quizQuestions.splice(idx, 1); 
  }

  async function addAnswer(event: Event, question: QuizQuestion) {
    event.preventDefault();
    question.answers.push({ text: "", is_correct: false })
  }

  async function setIsCorrect(question: QuizQuestion ,answer: QuestionAnswer) {
    question.answers.forEach(e => e.is_correct = false);
    answer.is_correct = true;
  }

  async function saveQuiz(event: Event) {
    event.preventDefault();
    await invoke("add_quiz", { quizName: quizName, questions: quizQuestions });
    // console.log("Cała tablica:", $state.snapshot(quizQuestions));
    
    // console.log("###", quizName)
    // quizQuestions.forEach(e => {
    //   console.log(e.text);
    //   e.answers.forEach(f => {
    //     console.log("===", f.text, "===", f.is_correct)
    //   });
    // });
  }

</script>

<main class="container">
  <div class="sidebar">
    <div class="menu-header">Menu</div>
    
    <div class="btn-group">
      <button class="menubtn" onclick={newQuiz}>New Quiz</button>
      <button class="menubtn" onclick={allQuizzes}>All Quizzes</button>
      <button class="menubtn" onclick={toogleTheme}>{isDarkMode ? 'Dark' : 'Light'}</button>
      <button class="menubtn" onclick={quit}>Quit</button>
    </div>
  </div>

  <div class="content">
    {#if currentView == "list"}
    <div class="quizz-grid">
      {#each quizzes as quiz}
        <div class="quizz-card">
          <div class="card-text">
            <span class="quiz-name">halo co tam jak czi mija zycie</span>
            <span class="quiz-meta">Q[0] GRUG</span>
          </div>
          <div class="quiz-edit-container">
            <button class="btn-start action-btn">Start</button>
            <button class="btn-edit action-btn">Edit</button>
            <button class="btn-delete action-btn">Delete</button>
          </div>
        </div>
      {/each}
    </div>
    {:else if currentView == "add"}
      <div class="add-quiz-container">
      <h2 class="section-title">Quiz Creator</h2>
      <input class="input-main" placeholder="quiz name..." bind:value={quizName}>
      <div class="questions-list">
        {#each quizQuestions as question (question)}
          <div class="question-card">
            
            <div class="question-header">
              <input class="input-main" placeholder="question..." bind:value={question.text}>
              <button class="btn-icon-delete" onclick={(e) => removeQuestion(e, question)}>✕</button>
            </div>
            <div class="answers-list">
              {#each question.answers as answer (answer)}
                <div class="answer-row">
                  <input 
                    type="radio" 
                    class="radio-custom" 
                    name="ans-{question.text}" 
                    onchange={() => setIsCorrect(question, answer)}>

                  <input class="input-sub" placeholder="answer..." bind:value={answer.text}>
                  <button class="btn-icon-small" onclick={(e) => removeAnswer(e, question, answer)}>✕</button>
                </div>
              {/each}
              <button class="menubtn btn-primary" onclick={(e) => addAnswer(e, question)}>+</button>
              </div>
          </div>
        {/each}
      </div>

      <div class="action-bar">
        <button class="menubtn btn-secondary" onclick={addQuestion}>+ Add question</button>
        <button class="menubtn btn-primary" onclick={saveQuiz}>Save Quiz</button>
      </div>
    </div>
    {/if}
  </div>
</main>

<style>

.add-quiz-container {
  max-width: 800px; 
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  gap: 30px;
  padding-bottom: 80px; 
}

.section-title {
  text-align: center;
  color: var(--text-color);
  margin-bottom: 10px;
}

.questions-list {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.question-card {
  background-color: var(--bg-card); 
  padding: 25px;
  border-radius: 16px;
  box-shadow: 0 4px 12px rgba(0,0,0,0.08);
  border: 1px solid transparent;
  transition: transform 0.2s ease;
  position: relative;
}

:global(body:not(.dark-mode)) .question-card {
  border: 1px solid #e0e0e0;
}

.question-header {
  display: flex;
  align-items: center;
  gap: 15px;
  margin-bottom: 20px;
  border-bottom: 1px solid var(--bg-sidebar);
  padding-bottom: 15px;
}

input {
  background-color: transparent;
  color: var(--text-color);
  border: 1px solid #ccc;
  border-radius: 8px;
  transition: all 0.2s;
}

input:focus {
  outline: none;
  border-color: var(--btn-hover);
  box-shadow: 0 0 0 3px rgba(57, 108, 216, 0.2);
}

.input-main {
  flex: 1;
  padding: 12px 15px;
  font-size: 1.1rem;
  font-weight: 600;
  border: 1px solid transparent;
  background-color: rgba(128, 128, 128, 0.05);
}

.input-sub {
  flex: 1;
  padding: 8px 12px;
  font-size: 0.95rem;
  border: 1px solid var(--bg-sidebar);
}

.answers-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
  padding-left: 10px; 
}

.answer-row {
  display: flex;
  align-items: center;
  gap: 12px;
}

.radio-custom {
  transform: scale(1.2); 
  cursor: default;
}

.btn-icon-delete {
  background: rgba(255, 0, 0, 0.1);
  color: #d63031;
  border: none;
  width: 36px;
  height: 36px;
  border-radius: 8px;
  font-weight: bold;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.btn-icon-delete:hover {
  background: #d63031;
  color: white;
}

.btn-icon-small {
  background: transparent;
  color: #aaa;
  border: none;
  font-size: 1.2rem;
  cursor: pointer;
  padding: 5px;
  border-radius: 50%;
}

.btn-icon-small:hover {
  color: #d63031;
  background-color: rgba(255,0,0,0.05);
}

.action-bar {
  display: flex;
  justify-content: flex-end; 
  gap: 15px;
  margin-top: 20px;
  padding-top: 20px;
  border-top: 2px dashed #ccc;
}

.btn-primary {
  background-color: var(--color-success);
  color: white;
  padding: 12px 30px;
}

.btn-secondary {
  background-color: var(--bg-sidebar);
  border: 1px solid #ccc;
}

.quizz-card {
    background-color: var(--bg-card);
    color: var(--text-color);
    border-radius: 12px;
    padding: 20px; 
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    box-shadow: 0 2px 5px rgba(0,0,0,0.05); 
    transition: all 0.2s ease;
    border: 1px solid transparent;
    min-height: 180px;
  }

.quizz-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 20px;
    padding-bottom: 20px;
}

.action-btn:hover {
    opacity: 0.9;
  }

  .action-btn:active {
    transform: scale(1.18); 
  }

.card-text {
    margin-bottom: 20px; 
  }

.quiz-name {
    display: block; 
    font-size: 1.25rem; 
    font-weight: 700;
    line-height: 1.4;
    margin-bottom: 8px; 
    white-space: normal;
    overflow-wrap: break-word;
  }

.quiz-meta {
    display: block;
    font-size: 0.85rem;
    font-weight: 500;
    color: var(--text-meta); 
    text-transform: uppercase; 
    letter-spacing: 0.5px;
  }

.btn-start { background-color: var(--color-success); }
.btn-edit  { background-color: var(--color-warning); color: #333; }
.btn-delete { background-color: var(--color-danger); }

.quiz-edit-container {
    display: flex;
    gap: 10px; 
    margin-top: auto; 
  }

.quizz-card:hover {
    transform: translateY(-4px);
    box-shadow: 0 10px 20px rgba(0,0,0,0.15);
  }

.action-btn {
    flex: 1;
    padding: 8px 0;
    border: none;
    border-radius: 6px;
    font-weight: 600;
    font-size: 0.9rem;
    cursor: pointer;
    color: white;
    transition: opacity 0.2s ease, transform 0.1s ease;
  }


  :global(body) {
    margin: 0;
    padding: 0;
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    transition: background-color 0.3s ease, color 0.3s ease;
  }

  :root {
    --bg-app: #f6f6f6;
    --bg-sidebar: #e0e0e0;
    --bg-content: #ffffff;
    --bg-card: #e0e0e0;
    
    --text-color: #0f0f0f;
    --text-meta: #666666; 

    --btn-bg: #ffffff;
    --btn-text: #0f0f0f;
    --btn-hover: #396cd8;
    --btn-hover-text: #ffffff;
    /* Action Colors */
    --color-success: #2ecc71;
    --color-warning: #f1c40f; 
    --color-danger:  #e74c3c; 
  }

  /* dark-mode def*/
  :global(body.dark-mode) {
    --bg-app: #2f2f2f;
    --bg-sidebar: #1a1a1a;
    --bg-content: #242424;
    --bg-card: #333333; 

    --text-color: #f6f6f6;
    --text-meta: #aaaaaa;

    --btn-bg: #444444;
    --btn-text: #ffffff;  
  }

  .container {
    display: flex;
    width: 100vw;
    height: 100vh;
    overflow: hidden;
    color: var(--text-color);
    background-color: var(--bg-app);
  }

  .sidebar {
    flex: 0 0 20%;
    background-color: var(--bg-sidebar);
    display: flex;
    flex-direction: column;
    padding: 20px;
    box-shadow: 2px 0 5px rgba(0,0,0,0.1);
    z-index: 10;
  }

  .menu-header {
    font-size: 1.5rem;
    font-weight: bold;
    margin-bottom: 30px;
    text-align: center;
  }

  .btn-group {
    display: flex;
    flex-direction: column;
    gap: 15px;
  }

  .content {
    flex: 1;
    background-color: var(--bg-content);
    padding: 40px;
    overflow-y: auto;
  }

  .menubtn {
    padding: 12px 20px;
    font-size: 1rem;
    cursor: pointer;
    background-color: var(--btn-bg);
    color: var(--btn-text);
    border: 1px solid transparent;
    border-radius: 8px;
    font-weight: 500;
    transition: all 0.2s ease;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  }

  .menubtn:hover {
    background-color: var(--btn-hover);
    color: var(--btn-hover-text);
    transform: translateY(-2px);
    box-shadow: 0 4px 8px rgba(0,0,0,0.2);
  }
</style>