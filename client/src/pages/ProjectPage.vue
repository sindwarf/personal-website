<template>
  <q-page class="q-pa-md">
    <div class="row">
      <div class="col-10 offset-1">
        <div class="text-h4 q-mb-md">Projects And Experience</div>

        <q-card v-for="project in projectList" :key="project.title" class="q-mb-md">
          <q-card-section>
            <a class="github-link" :href="project.link" target="_blank">
              <div class="text-h6">{{ project.title }}</div>
              <p>{{ project.description }}</p>
            </a>
          </q-card-section>
        </q-card>

        <!-- <div class="text-h4 q-mt-xl q-mb-md">Github Projects</div> -->

        <q-card v-for="project in githubProjects" :key="project.title" class="q-mb-md">
          <q-card-section>
            <div class="text-h6">
              <a class="github-link" :href="project.link" target="_blank">
                {{ project.title }}
              </a>
            </div>
            <p>{{ project.description }}</p>
          </q-card-section>
        </q-card>
      </div>
    </div>
  </q-page>
</template>


<script>
import { defineComponent, ref } from 'vue';

export default defineComponent({
  name: 'ProjectPage',

  setup() {
    const projectList = ref([
      {
        title: 'Angular and Django Web App',
        description: `I created a small web application that used Angular as the front end and Django as the back end. It allowed for users to log in and manage users with different permission levels. Certain users could create inventory items and print off labels as PDF's from the item information. It was deployed using Gunicorn and nginx.`
      },
      {
        title: 'Python Library for Document Generation',
        description: `There was a need to generate large documents in my workplace, while implementing definitions from existing documents. I wrote a library that utilized the pythondocx library to parse requirement documents and output a document in .docx format based on user input. This grew to become a markup type language that can use files like templates in several parts of the document and take in user defined styles.`
      },
      {
        title: 'Secturion Website',
        description: `Migrated an existing react based web application into Next.js in order to utilize static site generation. This increased the performance of the website(based upon lighthouse in google chrome) by over 20% and allowed us to migrate to a host that costed less monthly. Additionally, I improved the SEO and mobile formats of the webpage.`,
        link: "https://www.secturion.com/"
      },
      {
        title: 'Maintained Web Server on Embedded Hardware',
        description: `I regularly maintain a web server that uses a c library named CivetWeb on embedded hardware. Improved the data sanitization and error checking when making queries to the Redis database. Implemented several features including a server rack identifier, and improved user permission checking.`
      },
      {
        title: 'RVPark',
        description: 'A web application for managing RV parks. This is a work in progress.'
      }
    ]);

    const githubProjects = ref([
      {
        title: "E-Commerce Website",
        description: "Tech stack", 
        link: 'https://github.com/Armani-SDC/e-commerce-website',
      },
      {
        title: "This WebSite",
        description: "Tech stack", 
        link: 'https://github.com/sindwarf/personal-website',
      },
      {
        title: "My Van Dashboard",
        description: "What it is, tech stack", 
        link: 'https://github.com/sindwarf/VanMadeDashboard',
      }
      
      
    ]);

    return {
      projectList,
      githubProjects
    };
  }
});
</script>