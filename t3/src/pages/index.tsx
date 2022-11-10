import { type User, type Topic } from "@prisma/client";
import { type NextPage } from "next";
import Link from "next/link";
import React from "react";
import { trpc } from "../utils/trpc";

const Home: NextPage = () => {
  const topics = trpc.fizika.get_topics.useQuery("Fizika");
  const by_year: Record<number, (Topic & {
    authors: User[];
  })[]> = {}

  topics.data?.forEach((topic) => {
    if (!Array.isArray(by_year[topic.year])) {
      by_year[topic.year] = []
    }

    const x = by_year[topic.year]
    if (Array.isArray(x))
      x.push(topic)
  })

  return <div className="h-full w-full text-white">
    {(Object.keys(by_year).length != 0) ? (
      <section
        className="bg-black"
      >{Object.keys(by_year).map((year_str, i) => {
        const year = parseInt(year_str)
        const topics = by_year[year];
        const elem = topics?.map((topic, j) => {
          // grid grid-cols-2 gap-10
          return (
            <div
              className="first:mt-0 last:mb-0 bg-black border border-[rgba(255,255,255,0.12)] rounded-sm p-10 shadow-lg"
              key={j}

            >
              <Link href={`${topic.id}`}>
                <p>{topic.title}</p>
                {topic.authors.map(author => (
                  <p key={author.id}>{author.name}</p>
                ))}
              </Link>
            </div>
          )
        })

        let leto = ""
        switch (year) {
          case 0:
            leto = "Druge vsebine";
            break;
          case 1:
            leto = "Prvi letnik";
            break;
          case 2:
            leto = "Drugi letnik";
            break;
          case 3:
            leto = "Tretji letnik";
            break;
          case 4:
            leto = "Četrti letnik";
            break;
          default:
            leto = "NAPAKA";
            break;
        }

        return (
          <div
            key={i}
          >
            <h2 className="text-7xl">{leto}</h2>
            <hr />
            {elem}
          </div>
        )
      })}</section>
    ) : <p>Loading ...</p>}
  </div>
}

export default Home;