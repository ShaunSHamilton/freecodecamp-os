async function main() {
  const stdin = await new Promise((resolve, reject) => {
    let data = "";
    process.stdin.on("data", (chunk) => (data += chunk));
    process.stdin.on("end", () => resolve(data.trim()));
    process.stdin.on("error", reject);
  });

  try {
    eval(stdin);
  } catch (error) {
    console.error(JSON.stringify(error, null, 2));
  }
}

main();
