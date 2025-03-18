const color = document.getElementById('color')
const colorPicker = document.getElementById('color-picker').parentElement
const pickColor = document.getElementById('pick-color')

pickColor.onclick = () => {
    colorPicker.style.visibility = 'hidden'
    color.classList.remove('selected')
}
