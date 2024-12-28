const fastify = require('fastify')({ logger: true });

let screenshotData = null;

fastify.get('/', async (request, reply) => {
  return { message: 'screen share server' };
});

fastify.post('/screenshot', async (request, reply) => {
  screenshotData = request.body; // Store the binary data
  return 'uploaded';
});

fastify.get('/screenshot', async (request, reply) => {
  if (screenshotData) {
    reply.header('Content-Type', 'application/octet-stream');
    return screenshotData; 
  } else {
    reply.status(404).send('error');
  }
});

const start = async () => {
  const PORT = process.env.PORT || 3000;
  const HOST = '0.0.0.0'; 
  try {
    await fastify.listen({ port: PORT, host: HOST });
    console.log(`Server is running on http://${HOST}:${PORT}`);
  } catch (err) {
    fastify.log.error(err);
    process.exit(1);
  }
};

start();
