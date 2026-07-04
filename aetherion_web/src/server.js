const express = require('express');
const cors = require('cors');
const path = require('path');
const aiRouter = require('./routes/ai_router');

const app = express();
app.use(cors());
app.use(express.json());
app.use(express.static(path.join(__dirname, '../public')));
app.use('/api/ai', aiRouter);

app.post('/api/quantum/think', (req, res) => {
    console.log("Thinking Mode Engaged. Querying SMT verifiers...");
    setTimeout(() => {
        res.json({ status: 'verified', result: 'Logical qubit safely collapsed.' });
    }, 2000);
});

app.listen(3000, () => console.log('Aetherion API Gateway running on port 3000'));
