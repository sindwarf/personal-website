<template>
  <q-page class="flex flex-center">
    <!-- List of projects -->
  </q-page>
</template>


<script>
/*
Angular and Django Web App
Python Library for Document Generation
Secturion Website
Maintained Web Server on Embedded Hardware
Vue2 to Vue3 Migration
*/
/**
 * Angular and Django Web App
I created a small web application that used Angular as the front end and Django as the back end. It allowed for users to log in and manage users with different permission levels. Certain users could create inventory items and print off labels as PDF's from the item information. It was deployed using Gunicorn and nginx.
 */
/**
 * Python Library for Document Generation
There was a need to generate large documents in my workplace, while implementing definitions from existing documents. I wrote a library that utilized the pythondocx library to parse requirement documents and output a document in .docx format based on user input. This grew to become a markup type language that can use files like templates in several parts of the document and take in user defined styles.
 */
/**
 * Secturion Website
Migrated an existing react based web application into Next.js in order to utilize static site generation. This increased the performance of the website(based upon lighthouse in google chrome) by over 20% and allowed us to migrate to a host that costed less monthly. Additionally, I improved the SEO and mobile formats of the webpage.
 */
/**
 * Maintained Web Server on Embedded Hardware
I regularly maintain a web server that uses a c library named CivetWeb on embedded hardware. Improved the data sanitization and error checking when making queries to the Redis database. Implemented several features including a server rack identifier, and improved user permission checking.
 */
/**
 * Vue2 to Vue3 Migration
Migrated a large scale web application from Vue2 to Vue3. This involved upgrading all javascript libraries and updating breaking features for all associated libraries.
 */
/**
 * RVPark 
 */
/**
  Github Projects
  https://github.com/Armani-SDC/e-commerce-website
  https://github.com/sindwarf/personal-website 
  https://github.com/sindwarf/VanMadeDashboard 
 */
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'IndexPage'
});
</script>