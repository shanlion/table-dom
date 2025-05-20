<template>
  <canvas ref="canvasRef"></canvas>
</template>

<script setup>
import { onMounted, onUnmounted, ref } from 'vue'

const canvasRef = ref(null)
const rowHeight = 20
const columnWidths = [50, 150, 50]
const totalRows = 1000000 // 模拟一千万行可以扩展到这个数量级

const data = Array.from({ length: totalRows }, (_, i) => ({
  id: i,
  name: `Name ${i}`,
  age: Math.floor(Math.random() * 100),
}))

const render = () => {
  const canvas = canvasRef.value
  const ctx = canvas.getContext('2d')
  const scrollTop = window.scrollY

  canvas.width = window.innerWidth
  canvas.height = window.innerHeight

  const startRow = Math.floor(scrollTop / rowHeight)
  const visibleRowCount = Math.ceil(canvas.height / rowHeight)

  ctx.clearRect(0, 0, canvas.width, canvas.height)
  ctx.font = '14px sans-serif'
  ctx.textBaseline = 'middle'

  for (let i = 0; i < visibleRowCount; i++) {
    const rowIndex = startRow + i
    const row = data[rowIndex]
    if (!row) continue
    const y = i * rowHeight

    ctx.strokeRect(0, y, canvas.width, rowHeight)
    ctx.fillText(String(row.id), 5, y + rowHeight / 2)
    ctx.fillText(row.name, columnWidths[0] + 5, y + rowHeight / 2)
    ctx.fillText(String(row.age), columnWidths[0] + columnWidths[1] + 5, y + rowHeight / 2)
  }
}

const onScroll = () => {
  render()
}

onMounted(() => {
  render()
  window.addEventListener('scroll', onScroll)

  // 设置 body 高度让页面可滚动
  document.body.style.height = `${totalRows * rowHeight}px`
})

onUnmounted(() => {
  window.removeEventListener('scroll', onScroll)
})
</script>

<style scoped>
canvas {
  position: fixed;
  top: 0;
  left: 0;
  z-index: 1;
}
body {
  margin: 0;
}
</style>