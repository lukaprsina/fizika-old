import { NextPage, } from 'next';
import { MeiliSearch } from 'meilisearch'
import movies from '../components/movies.json'

const client = new MeiliSearch({ host: 'http://localhost:7700' })
client.index('movies').addDocuments(movies)
    .then((res) => console.log(res))


const Database: NextPage = () => {
    client.index('movies').search('botman').then((res) => console.log(res))
    return <h1>Test</h1>
}

export default Database;