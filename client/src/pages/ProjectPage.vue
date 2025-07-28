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
        title: 'Python Library for Document Generation',
        description: `In my previous role, I built a Python library—leveraging python-docx—to automate the creation of large, structured Word documents. It parses existing requirement files, merges in user-provided data, and outputs a fully formatted .docx based on your inputs. Over time, I expanded it into a lightweight markup language. You can inject template fragments anywhere in the document, define custom styles, and supply dynamic content blocks—turning a one-off script into a reusable, extensible document-generation engine.
`
      },
      {
        title: 'Secturion Website',
        description: `Migrated an existing react based web application into Next.js in order to utilize static site generation. This increased the performance of the website(based upon lighthouse in google chrome) by over 20% and allowed us to migrate to a host that costs less monthly. Additionally, I improved the SEO and mobile formats of the webpage. I continue to maintain and improve this website.`,
        link: "https://www.secturion.com/"
      },
      {
        title: 'Maintained Web Server on Embedded Hardware',
        description: `I regularly maintain a web server that uses a c library named CivetWeb on embedded hardware. Improved the data sanitization and error checking when making queries to Redis. Implemented several features including a server rack identifier, and improved user permission checking.`
      },
      {
        title: 'Desert Eagle RV Park',
        description: 'This is a project that I contributed to during my last two semesters at Weber State University. This is a full reservation system for browsing and reserving campsites at the Desert Eagle RV Park. The features are extensive including admin management for all sites, report generation, and dynamic fees. This is a work in progress and will be releasing soon.'
      }
    ]);

    const githubProjects = ref([
      {
        title: "E-Commerce Website",
        description: "Our team wanted to create a functioning retail web-portal with the goals of providing a modern and smooth shopping experience. The main components of this web-page include a Product Overview, Recommended Items, Q&A, and Ratings and Reviews. This was built using node.js with react and express.js. This was one of the first projects I was a part of and kick started my web dev experience.", 
        link: 'https://github.com/Armani-SDC/e-commerce-website',
      },
      {
        title: "This WebSite",
        description: "My personal website that you are viewing right now is built using Vue and axum_server. For the javascript front end in Vue I am using the Quasar for my design library. I chose to use axum_server to further deepen my knowledge of rust and utilize the performance advantage over someting like express.js.", 
        link: 'https://github.com/sindwarf/personal-website',
      },
      {
        title: "My Van Dashboard",
        description: "This was a project I worked on to use while I travelled in a van for a few years. The intention was to create a dashboard where I could view current weather and statistics provided by my in-house electrical system. This is a web based application using React and Express.js.", 
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