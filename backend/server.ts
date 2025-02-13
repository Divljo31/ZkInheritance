import express from 'express';
import cors from 'cors';
import { promises as fs } from 'fs';
import path from 'path';

const app = express();

app.use(cors());
app.use(express.json());

app.post('/api/testaments', async (req, res) => {
  try {
    const testamentData = req.body;
    const filePath = path.join(__dirname, '../database', `${testamentData.id}.json`);
    
    // Create the database directory if it doesn't exist
    await fs.mkdir(path.join(__dirname, '../database'), { recursive: true });
    
    // Save the testament data as a JSON file
    await fs.writeFile(filePath, JSON.stringify(testamentData, null, 2));
    
    res.status(201).json({ message: 'Testament saved successfully', id: testamentData.id });
  } catch (error) {
    console.error('Error saving testament:', error);
    res.status(500).json({ error: 'Failed to save testament' });
  }
});

const PORT = process.env.PORT || 3000;
app.listen(PORT, () => {
  console.log(`Server running on port ${PORT}`);
});