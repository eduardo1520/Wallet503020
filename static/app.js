function formatCurrency(value) {
  return 'R$ ' + value.toFixed(2).replace('.', ',');
}

async function calculate() {
  const income = parseFloat(document.getElementById('income').value);
  if (isNaN(income) || income <= 0) { alert('Digite um valor válido!'); return; }

  try {
    const response = await fetch('/calculate?income=' + income);
    const data = await response.json();
    document.getElementById('needs').textContent = formatCurrency(data.needs.amount);
    document.getElementById('wants').textContent = formatCurrency(data.wants.amount);
    document.getElementById('savings').textContent = formatCurrency(data.savings.amount);
    document.getElementById('result').style.display = 'block';
  } catch (error) {
    alert('Erro: ' + error.message);
  }
}

document.getElementById('btn').addEventListener('click', calculate);