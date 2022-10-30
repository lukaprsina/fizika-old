const { PrismaClient } = require('@prisma/client')

async function seed() {
    const prisma = new PrismaClient();
    await prisma.post.deleteMany();
    await prisma.user.deleteMany();
    await prisma.user.create({
        data: {
            name: "Alice",
            email: "alice@prisma.io"
        }
    });
}

seed();