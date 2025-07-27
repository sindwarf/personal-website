<template>
  <!-- <q-page class="flex flex-center align-center q-m-auto"> -->
    <!-- Put in image of me -->
    <!-- <div class="q-mx-auto bg-secondary">
    </div>
    <q-btn to="/projects" label="Projects" />
    <q-btn href="https://www.linkedin.com/in/jonathansindorf/" label="Linked In" target="_blank" />
    <q-btn href="https://github.com/sindwarf" label="Github" target="_blank" /> -->
      <q-page class="row items-center justify-center q-pa-md">

    <q-card class="col-12 col-sm-8 col-md-6 col-lg-4 q-pa-lg">

      <!-- placeholder image -->
      <q-img
        src="https://placehold.co/400x200"
        alt="Image placeholder"
        class="q-mb-md"
        ratio="4/2"
      />

      <!-- centered paragraph -->
      <q-card-section class="text-center">
        <p class="q-mb-lg">

          Hello, my name is Jonathan Sindorf. I am a 27 year old software engineer from Alaska that is currently living in Utah. I have a beautiful wife and 1 year old daughter. I am currently employed full time at Secturion Systems, Inc. where I work on embedded systems and full stack web developement. I am also attending Weber State University full time to get my Bachelors Degree in Computer Science. Welcome to my site!
        </p>
      </q-card-section>

      <!-- buttons in a row -->
      <q-card-actions class="justify-center">
        <q-btn
          flat
          outline
          icon="mdi-linkedin"
          label="LinkedIn"
          class="q-mx-sm"
          href="https://www.linkedin.com/in/jonathansindorf/"
          target="_blank"
        />
        <q-btn
          flat
          outline
          icon="mdi-github"
          label="GitHub"
          class="q-mx-sm"
          href="https://github.com/sindwarf"
          target="_blank"
        />
        <q-btn
          flat
          outline
          icon="work"
          label="projects"
          class="q-mx-sm"
          to="/projects"
        />
      </q-card-actions>

    </q-card>

  </q-page>
</template>

<script>
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'IndexPage',

  setup() {
    
  }
});
</script>
