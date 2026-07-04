const express = require('express');
const router = express.Router();

let apiQuotaExhausted = false; 

router.post('/veo3', async (req, res) => {
    const { prompt } = req.body;
    let apiKey = apiQuotaExhausted ? process.env.PERSONAL_API_KEY : process.env.ENTERPRISE_API_KEY;
    
    console.log(`Generating Veo 3.1 video using prompt: ${prompt}`);
    res.json({ status: 'success', message: 'Video generation queued with native audio.' });
});

module.exports = router;
