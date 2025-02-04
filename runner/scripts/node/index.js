async function main() {
  const stdin = await new Promise((resolve, reject) => {
    let data = "";
    process.stdin.on("data", (chunk) => (data += chunk));
    process.stdin.on("end", () => resolve(data.trim())); // Trim to remove any accidental newline issues
    process.stdin.on("error", reject);
  });

  try {
    const testMeta = JSON.parse(stdin); // Correctly parse the JSON input
    console.log(JSON.stringify(testMeta, null, 2));

    for (const test of testMeta.tests) {
      console.log(`Running test: ${test.text}`);

      try {
        eval(test.code); // Executes the test code (Use with caution!)
        console.log(`Test ${test.id} passed.`);
      } catch (error) {
        console.error(`Test ${test.id} failed:`, error);
      }
    }
  } catch (error) {
    console.error("Failed to parse input JSON:", error);
    process.exit(1);
  }
}

main();
