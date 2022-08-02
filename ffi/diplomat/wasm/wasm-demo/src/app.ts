import { format } from "./index";

function submit(input: number): void {
    const my_input = document.getElementById('my-input');
    try {
        const fmt = format(input);
        document.getElementById('text-box').innerHTML = fmt;
        my_input.style.background = 'transparent';
    } catch (e) {
        document.getElementById('text-box').innerHTML = e.message;
        my_input.style.background = 'red';
    }
}

const my_input = document.getElementById('my-input') as HTMLInputElement | null;
my_input?.addEventListener('input', () => submit(+my_input.value));
