<template>
  <q-layout view="hHh lpR fFf">
    <q-header flat >
      <q-toolbar>
        <q-space></q-space>

        <q-toolbar-title>
          <a
            href="/#"
           >
            Jonathan Sindorf
          </a>
        </q-toolbar-title>

        <div>Software Engineer</div>
        <q-space></q-space>
      </q-toolbar>
    </q-header>

    <q-page-container >
      <router-view />
    </q-page-container>
    <q-footer flat class="flex flex-center">
      <div id="clock">
        <p class="date">{{ date }}</p>
        <p class="time">{{ time }}</p>
      </div>
    </q-footer>
  </q-layout>
</template>

<script>
import { defineComponent, ref, onMounted, onUnmounted } from 'vue'

export default defineComponent({
  name: 'MainLayout',

  setup () {
    const time = ref('')
    const date = ref('')

    const week = ['SUN', 'MON', 'TUE', 'WED', 'THU', 'FRI', 'SAT']

    const zeroPadding = (num, digit) => {
      return String(num).padStart(digit, '0')
    }

    const updateTime = () => {
      const cd = new Date()
      time.value = `${zeroPadding(cd.getHours(), 2)}:${zeroPadding(cd.getMinutes(), 2)}:${zeroPadding(cd.getSeconds(), 2)}`
      date.value = `${zeroPadding(cd.getFullYear(), 4)}-${zeroPadding(cd.getMonth() + 1, 2)}-${zeroPadding(cd.getDate(), 2)} ${week[cd.getDay()]}`
    }

    let timerID
    onMounted(() => {
      updateTime()
      timerID = setInterval(updateTime, 1000)
    })

    onUnmounted(() => {
      clearInterval(timerID)
    })

    return {
      time,
      date
    }
  }
})
</script>