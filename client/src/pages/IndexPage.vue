<template>
  <!-- <q-page class="flex flex-center align-center q-m-auto"> -->
    <!-- Put in image of me -->
    <!-- <div class="q-mx-auto bg-secondary">
    </div>
    <q-btn to="/projects" label="Projects" />
    <q-btn href="https://www.linkedin.com/in/jonathansindorf/" label="Linked In" target="_blank" />
    <q-btn href="https://github.com/sindwarf" label="Github" target="_blank" /> -->
  <q-page class="row items-center justify-center index-page">

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
          Welcome to my personal website. I am a software engineer based in Ogden, Utah. I work full time at Secturion Systems where I fulfill a variety of roles. I frequently work on embedded systems and full stack web developement. I have also written my fair share of tools for other developers within the company. I am currently attending Weber State University full timae to get my Bachelors in Computer Science.
        </p>
        
        <p class="q-mb-lg">
          In my free time I like to explore open source projects and see what kind of interesting things people are working on. I have recently acquired a <a href="https://pine64.org/devices/pinetime/">PineTime Watch</a> and am working on creating a custom watchface for <a href="https://github.com/InfiniTimeOrg/InfiniTime">Infinitime</a>
          I am also beginning to dive into the world of cybersecurity. I am going to be attending <a href="https://www.saintcon.org/">Saintcon</a> in the fall. So if you're there say hi!
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
