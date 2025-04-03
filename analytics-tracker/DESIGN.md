# User stories (questions to answer using the project) - CORE 

## Content 

For a piece of content, what is the total and distribution of likes, views, shares, and comments across all platforms? On each platform, what is its title, hashtags, sponsorship status, ad status, date posted, and additional notes? 

For a piece of content, what is the production cost, revenue (ad revenue or sponsorship), and profit made per platform? 

How many pieces of content did I post in the last X days, months, years, per platform? 

Filtering and sorting (AND operation): 
- Sort content by highest views, highest revenue, alphabetic by title, and most recently posted (default). 
- Filter content by sponsored or not. 
- Range of views (lower and upper bound). 
- Range of revenue (lower and upper bound). 
- Select tags (AND or OR operation) - ex: "drawing tutorial", "DIY ideas", etc. 
- Search video by title containing keywords. 

Be able to add a new content, tracking its revenue/expenses and analytics on an arbitrary number of platforms. Be able to delete an existing piece of content from the set of all content being tracked. 

## Analytics 

Be able to add a new platform on which content is posted to, to track content performance on that specific platform. 

Be able to see content performance (likes, views, shares, comments) separated by platform and aggregated across an arbitrary number of platforms - compare performance across X number of selected platforms. 

Compare content performance (likes, views, shares, comments) based on content type (defined by tags) - what type of content is most likely to go viral? 

What factors affect "virality"? Give basic feedback based on data. Ex: "Content with the theme X is more likely to go viral in the last month based on data from Y."

What type (based on tag) of content is most likely to go viral on each platform, historically? Display graph of tags vs views/likes/income for each piece of content. 

How does posting time affect a content's engagement? What is the optimal posting time for each platform? 

Select multiple (2 or more) pieces of content and see aggregated data analytics on only the selected items. Compare and contrast content performance across platforms. 

Allow either manual input for video analytics or automatic poll from platform (if API is available). 

Polling analytics from platform: 
- If video is older than X days and currently not viral, batch poll analytics from respective platforms. 
- If video is posted within last 24h, do more frequent polling. If video is posted within last X days, do semi-frequent polling. 
- If video is currently viral (regardless of posting date) do more frequent polling. 
- (Optional:) click the "update now" button to poll analytics for current video immediately. 
In this context, "virality" can be defined as "the difference between the last 2 poll results is greater than Y% of the total views". Condiferations: limited API tokens per day. 

Display total follower count per platform. 

## Monetization 

What is the total production cost, revenue (ad revenue or sponsorship), and profit made in the last X hours, days, months, years, per platform? 

What is my total revenue and profit over the last X days, months, years, per platform? On which videos did I spend the most on? 

## General 

Breakdown into Overview, Insights, Notes, Settings. Insights should contain further breakdown into Dashboard, Content, Analytics, and Monetization. 

Be able to make a new account or login, log out, change password, transfer account, and delete account. 

Be able to securely connect analytics tracker account to Instagram, YouTube, Facebook, TikTok, Snapchat, (or other platforms). Also allows for custom platforms (manual input) if API is not currently supported. 

## Generating content ideas (notes) 

Write down impulsive content ideas, or add annotations to a posted piece of content (what went well, what didn't, etc.). 

Schedule a posting reminder. Given a sponsored post (or usual post) to be posted on a specific day and time, set a reminder that will be emailed X minutes before the actual posting time. 

## User stories - non-priority 

Which piece(s) of content are currently going viral, by some definition of "virality"? 

Predict what type of content is most likely to go viral based on current time/season, trends, previous content data, last X videos posted, and viral content from top creators or creators in the same niche etc.

Who is my main audience on each platform (age, gender, geographical location), and what similar content do they watch? 

What is the click-through or conversion rate from affiliate links in video descriptions on a sponsored post? 

Export all data as PDF/CSV. 

Generate new content ideas based on type of content posted and top performing content from other creators. 

Multiple accounts support, for creators with different brands or channels. 

Share account access with a team, with view/edit permissions. 

Track and log changes and updates made to account - dates and times that new content was added, etc. 

Gauge how positive or negative the audience reaction is on each post based on comments/keywords. 

Retrieve comments to parse number of @ tags (as one potential measure of interest in content). 

Given a video's current analytics, extrapolate information about its predicted analytics - given a video with X views/likes/comments/shares, how many views/likes/comments/shares will it have in Y hours/days? 

# TODO 

- Go through notes on tablet 
