const { PrismaClient } = require('@prisma/client')

const prisma = new PrismaClient()

async function main() {
  try {
    await prisma.user.deleteMany();
    await prisma.verification.deleteMany();
  } catch (err) {
    console.log(err.toString());
  }
}

main();
