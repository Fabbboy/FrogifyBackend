const loginForm = document.getElementById("login-form");
const registerForm = document.getElementById("register-form");

const loginEmail = document.getElementById("login-email");
const loginPassword = document.getElementById("login-password");

const registerEmail = document.getElementById("register-email");
const registerPassword = document.getElementById("register-password");

loginForm.addEventListener("submit", async (e) => {
    e.preventDefault();
    const email = loginEmail.value;
    const password = loginPassword.value;

    const response = await fetch("http://localhost:4499/auth/login", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            method: "Default",
            usermail: email,
            password: password,
        }),
    });

    const data = await response.json();
    if (data.success) {
        localStorage.setItem("usermail", email);
        localStorage.setItem("userToken", data.userToken);
        alert("Login successful!");
    } else {
        alert("Login failed: " + data.message);
    }
});

registerForm.addEventListener("submit", async (e) => {
    e.preventDefault();
    const email = registerEmail.value;
    const password = registerPassword.value;

    const response = await fetch("http://localhost:4499/auth/register", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            username: email,
            usermail: email,
            password: password,
        }),
    });

    const data = await response.json();
    if (data.success) {
        alert("Registration successful!");
        localStorage.setItem("usermail", email);
        localStorage.setItem("userToken", data.userToken);
    } else {
        alert("Registration failed: " + data.message);
    }
});

(async () => {
    console.log("Auto login...");
    const email = localStorage.getItem("usermail");
    const token = localStorage.getItem("userToken");

    if (email && token) {
        const response = await fetch("http://localhost:4499/auth/login", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                method: "UserToken",
                usermail: email,
                userToken: token,
            }),
        });

        const data = await response.json();
        if (data.success) {
            alert("Auto login successful!");
        } else {
            alert("Auto login failed: " + data.message);
            localStorage.removeItem("usermail");
            localStorage.removeItem("userToken");
        }
    }
})();
