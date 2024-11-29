<script setup>
import { ref } from 'vue';
import { my_project_backend } from 'declarations/my_project_backend/index';
let displayChat = ref('');

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const msg = target.querySelector('#msg').value;
  //displayChat.value = msg;
  await my_project_backend.save_msg(msg);
  await getMsgs();
}

async function getMsgs() {
  const chat = await my_project_backend.get_chat();
  console.log(chat);
  displayChat.value = chat;
}

getMsgs();
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    <form action="#" @submit="handleSubmit">
      <label for="msg">Enter your msg: &nbsp;</label>
      <input id="msg" alt="msg" type="text" />
      <button type="submit">Click Me!</button>
    </form>
    <section id="displayChat">
      <div v-for="msg in displayChat">{{ msg }}</div>
    </section>
  </main>
</template>
