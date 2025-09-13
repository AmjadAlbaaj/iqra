<script>
  let query = '';
  let results = [];
  let loading = false;
  let error = '';

  async function searchPackages() {
    loading = true;
    error = '';
    results = [];
    try {
      // TODO: Replace with real registry API endpoint
      const res = await fetch(`https://iqra-registry.example.com/packages?q=${encodeURIComponent(query)}`);
      if (!res.ok) throw new Error('فشل الاتصال بالسجل | Registry connection failed');
      results = await res.json();
    } catch (e) {
      error = e.message;
    } finally {
      loading = false;
    }
  }
</script>

<main>
  <h1>سجل مكتبات اقرأ | iqra Package Registry</h1>
  <input type="text" bind:value={query} placeholder="ابحث عن مكتبة... | Search for a package..." />
  <button on:click={searchPackages} disabled={loading}>بحث | Search</button>
  {#if loading}
    <p>جاري البحث... | Loading...</p>
  {/if}
  {#if error}
    <p style="color:red">{error}</p>
  {/if}
  {#if results.length}
    <ul>
      {#each results as pkg}
        <li>
          <strong>{pkg.name}</strong> - {pkg.description}
          <br />
          <span>المؤلف: {pkg.author} | Author: {pkg.author}</span>
        </li>
      {/each}
    </ul>
  {/if}
</main>

<style>
main {
  max-width: 600px;
  margin: 2rem auto;
  font-family: 'Segoe UI', Arial, sans-serif;
  background: #fff;
  padding: 2rem;
  border-radius: 8px;
  box-shadow: 0 2px 8px #eee;
}
h1 {
  text-align: center;
  margin-bottom: 2rem;
}
input {
  width: 70%;
  padding: 0.5rem;
  margin-right: 0.5rem;
}
button {
  padding: 0.5rem 1rem;
}
ul {
  margin-top: 2rem;
  padding-left: 1rem;
}
li {
  margin-bottom: 1.5rem;
  background: #f9f9f9;
  padding: 1rem;
  border-radius: 6px;
}
</style>
