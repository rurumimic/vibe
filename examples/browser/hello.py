from browser_use import Agent, Browser, ChatAnthropic
from dotenv import load_dotenv
import asyncio

load_dotenv()


async def main():
    browser = Browser(
        # headless=False,  # Show browser window
        headless=True,  # Show browser window
        window_size={"width": 1000, "height": 700},  # Set window size
    )
    llm = ChatAnthropic(
        model="claude-sonnet-4-0",
        temperature=0.0,
    )
    task = "Find the number 1 post on Show HN"
    agent = Agent(
        task=task,
        llm=llm,
        browser=browser,
    )

    await agent.run()


if __name__ == "__main__":
    asyncio.run(main())
