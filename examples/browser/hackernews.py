from browser_use import Agent, Browser, ChatAnthropic
from dotenv import load_dotenv
import asyncio

load_dotenv()


async def main():
    browser = Browser(
        headless=False,
        keep_alive=True,  # 중요: agent.run() 이후에도 브라우저 유지
        window_size={"width": 1000, "height": 700},
    )

    await browser.start()  # 명시적으로 브라우저 세션 시작

    llm = ChatAnthropic(
        model="claude-sonnet-4-0",
        temperature=0.0,
    )

    try:
        open_ask_agent = Agent(
            task="""
            Go to https://news.ycombinator.com/ask.
            Stop after the Ask HN page is opened.
            Do not click any post.
            """,
            llm=llm,
            browser=browser,
        )

        await open_ask_agent.run(max_steps=10)

        input(
            "\n브라우저에서 마음에 드는 Ask HN 게시글을 클릭한 뒤, "
            "터미널에서 Enter를 누르세요...\n"
        )

        summarize_agent = Agent(
            task="""
            Summarize the currently opened Hacker News post page in Korean.

            Include:
            1. Title
            2. Main content
            3. Short Korean summary
            """,
            llm=llm,
            browser=browser,
        )

        result = await summarize_agent.run(max_steps=20)

        print("\n=== SUMMARY ===")
        print(result)

        input("\n브라우저를 계속 열어둡니다. 종료하려면 Enter를 누르세요...\n")

    finally:
        # 정말 닫고 싶을 때만 호출
        await browser.stop()


if __name__ == "__main__":
    asyncio.run(main())

