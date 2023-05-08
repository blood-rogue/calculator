let equal_pressed = 0

let button_input = document.querySelectorAll(".input-button")

let input = document.getElementById("input")
let equal = document.getElementById("equal")
let clear = document.getElementById("clear")
let erase = document.getElementById("erase")

window.onload = () => {
    input.value = ""
}

window.addEventListener("keydown", (ev) => {
    switch (ev.key) {
        case "Enter":
        case "=":
            equal.click()
            break

        case "Delete":
            clear.click()
            break

        case "Backspace":
            erase.click()
            break

        case "0":
        case "1":
        case "2":
        case "3":
        case "4":
        case "5":
        case "6":
        case "7":
        case "8":
        case "9":
        case "+":
        case "-":
        case "*":
        case "/":
        case ".":
            input.value += ev.key

        default:
            break
    }
    if (ev.key === "Enter" || ev.key === "=") { equal.click() }
    else if (ev.key === "Delete") { clear.click() }
    else if (ev.key === "Backspace") { erase.click() }
    else if (ev.key.length == 1) input.value += ev.key
})

button_input.forEach((button_class) => {
    button_class.addEventListener("click", () => {
        if (equal_pressed == 1) {
            input.value = ""
            equal_pressed = 0
        }

        input.value += button_class.value
    })
})

const { invoke } = window.__TAURI__.tauri

equal.addEventListener("click", () => {
    equal_pressed = 1
    let inp = input.value

    invoke('parse_and_eval', { inp }).then(ret => input.value = ret)
})


clear.addEventListener("click", () => {
    input.value = ""
})

erase.addEventListener("click", () => {
    input.value = input.value.substr(0, input.value.length - 1)
})