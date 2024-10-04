import { createNewTask } from './createTask';

function generateRandomName(): string {
    const adjectives = ['Quick', 'Lazy', 'Sleepy', 'Noisy', 'Hungry'];
    const nouns = ['Fox', 'Dog', 'Cat', 'Mouse', 'Bear'];
    const adjective = adjectives[Math.floor(Math.random() * adjectives.length)];
    const noun = nouns[Math.floor(Math.random() * nouns.length)];
    const randomName = `${adjective}${noun}${Math.floor(Math.random() * 1000)}`;
    return randomName;
}

function startCreatingTasks() {
    setInterval(() => {
        const randomName = generateRandomName();
        console.log(`Creating new task with name: ${randomName}`);
        createNewTask(randomName);
    }, 5000);
}

// Start the process
startCreatingTasks();