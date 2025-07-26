<template>
  <q-layout view="lHh Lpr lFf">
    <q-header flat>
      <q-toolbar>

        <q-toolbar-title>
          Jonathan Sindorf
        </q-toolbar-title>

        <div>Quasar v{{ $q.version }}</div>
      </q-toolbar>
    </q-header>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script>
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'MainLayout',

  setup () {
    return {
    }
  }
})
</script>
