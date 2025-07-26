<template>
  <q-page class="flex flex-center">
    <!-- Put in image of me -->
    <p>
    Hello, my name is Jonathan Sindorf. I am a 27 year old software engineer from Alaska that is currently living in Utah. I have a beautiful wife and 1 year old daughter. I am currently employed full time at Secturion Systems, Inc. where I work on embedded systems and full stack web developement. I am also attending Weber State University full time to get my Bachelors Degree in Computer Science. Welcome to my site!
    </p>
    <q-btn to="/projects" label="Projects" />
    <q-btn href="https://www.linkedin.com/in/jonathansindorf/" label="Linked In" target="_blank" />
  </q-page>
</template>

<script>
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'IndexPage'
});
</script>
