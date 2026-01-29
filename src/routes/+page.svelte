<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  type QuizItem = {
    id: number;
    name: string;
  }
  
  let isDarkMode = $state(window.matchMedia('(prefers-color-scheme: dark)').matches);
  let quizzes: QuizItem[] = $state([]);
  let counter: number = 0;

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

  async function newquiz(event: Event) {
    event.preventDefault();
    quizzes.push({
      id: counter,
      name: "GRUG" + counter
    })
  }

  async function quit(event: Event) {
    event.preventDefault();
    console.log("Quit Clicked");
  }
</script>

<main class="container">
  <div class="sidebar">
    <div class="menu-header">Menu</div>
    
    <div class="btn-group">
      <button class="menubtn" onclick={newquiz}>New Quiz</button>
      <button class="menubtn" onclick={toogleTheme}>{isDarkMode ? 'Dark' : 'Light'}</button>
      <button class="menubtn" onclick={quit}>Quit</button>
    </div>
  </div>

  <div class="content">
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
        <!-- 
          
            
            
          </div>
            

            </div>
        </div> -->
    </div>
  </div>
</main>

<style>


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