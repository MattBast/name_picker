import { test, expect } from "@playwright/test";

test("homepage has title", async ({ page }) => {
  await page.goto("http://localhost:3000");
  await expect(page).toHaveTitle("Random.Pick");
});

test("homepage disappears when an item is added to list", async ({ page }) => {
  await page.goto("http://localhost:3000");
  await page.waitForLoadState("networkidle"); // Firefox requires a delay

  // the headings are visible when the page first opens
  await expect(page.getByText("Name Picker")).toBeVisible();
  await expect(
    page.getByText("Write a list of names and randomly select one."),
  ).toBeVisible();

  // add item to list
  await page.getByRole("button", { name: "Add a name" }).click();

  await expect(page.getByText("Name Picker")).toBeHidden();
  await expect(
    page.getByText("Write a list of names and randomly select one."),
  ).toBeHidden();
});

test("an item is added to the list when the add button is clicked", async ({
  page,
}) => {
  await page.goto("http://localhost:3000");
  await page.waitForLoadState("networkidle"); // Firefox requires a delay

  // make sure no cards are visible to the user when the page first opens
  await expect(page.getByTestId("card")).toHaveCount(0);

  // add item to list
  await page.getByRole("button", { name: "Add a name" }).click();

  // make sure one card is added when the button is clicked.
  await expect(page.getByTestId("card")).toHaveCount(1);
});

test("list items disappear when they are empty and blurred (not focussed)", async ({
  page,
}) => {
  await page.goto("http://localhost:3000");
  await page.waitForLoadState("networkidle"); // Firefox requires a delay

  // add item to list
  await page.getByRole("button", { name: "Add a name" }).click();

  // blur the cards input after the application has focussed on it
  await page
    .getByTestId("card")
    .first()
    .getByRole("textbox")
    .blur({ timeout: 1000 });

  // make sure no cards are visible to the user as the card should have deleted itself
  await expect(page.getByTestId("card")).toHaveCount(0);

  // make sure the headings have reappeared if there are no longer any cards visible
  await expect(page.getByText("Name Picker")).toBeVisible();
  await expect(
    page.getByText("Write a list of names and randomly select one."),
  ).toBeVisible();
});

test("list items remains visible after a name is added to it", async ({
  page,
}) => {
  await page.goto("http://localhost:3000");
  await page.waitForLoadState("networkidle"); // Firefox requires a delay

  // add item to list
  await page.getByRole("button", { name: "Add a name" }).click();

  // add a name to the textbox and then stop focussing on it.
  await page.getByTestId("card").first().getByRole("textbox").fill("name");
  await page.getByTestId("card").first().getByRole("textbox").blur();

  // make sure the card remains after the blur event
  await expect(page.getByTestId("card")).toHaveCount(1);
});

test("list items is deleted when its delete button is clicked", async ({
  page,
}) => {
  await page.goto("http://localhost:3000");
  await page.waitForLoadState("networkidle"); // Firefox requires a delay

  // add item to list
  await page.getByRole("button", { name: "Add a name" }).click();

  // add a name to the textbox and then stop focussing on it.
  await page.getByTestId("card").first().getByRole("textbox").fill("name");
  await page.getByTestId("card").first().getByRole("button").click();

  // make sure the card was deleted
  await expect(page.getByTestId("card")).toHaveCount(0);
});

test("list item is blurred and deleted when Escape key is pressed and the card is empty", async ({
  page,
}) => {
  await page.goto("http://localhost:3000");
  await page.waitForLoadState("networkidle"); // Firefox requires a delay

  // add item to list
  await page.getByRole("button", { name: "Add a name" }).click();

  // Press escape while focussed on the card. As the card is empty of text, it should
  // blur and then delete itself.
  await page.getByTestId("card").first().getByRole("textbox").press("Escape");

  // make sure the card was deleted
  await expect(page.getByTestId("card")).toHaveCount(0);
});

test("list item is blurred when Escape key is pressed", async ({ page }) => {
  await page.goto("http://localhost:3000");
  await page.waitForLoadState("networkidle"); // Firefox requires a delay

  // add item to list
  await page.getByRole("button", { name: "Add a name" }).click();

  // Add some text to the card and then press escape while focussed on the card.
  await page.getByTestId("card").first().getByRole("textbox").fill("Name 1");
  await page.getByTestId("card").first().getByRole("textbox").press("Escape");

  // make sure the card is no longer focussed
  await expect(page.getByTestId("card")).not.toBeFocused();
});

test("a new list item is created after user presses enter while focussed on an existing card", async ({
  page,
}) => {
  await page.goto("http://localhost:3000");
  await page.waitForLoadState("networkidle"); // Firefox requires a delay

  // add item to list
  await page.getByRole("button", { name: "Add a name" }).click();

  // Press enter while focussed on the first card.
  await page.getByTestId("card").first().getByRole("textbox").fill("name 1");
  await page.getByTestId("card").first().getByRole("textbox").press("Enter");

  // make sure the new card was created.
  await expect(page.getByTestId("card")).toHaveCount(2);
});

test("a new list item is created after user presses tab while focussed on an existing card", async ({
  page,
}) => {
  await page.goto("http://localhost:3000");
  await page.waitForLoadState("networkidle"); // Firefox requires a delay

  // add item to list
  await page.getByRole("button", { name: "Add a name" }).click();

  // Press enter while focussed on the first card.
  await page.getByTestId("card").first().getByRole("textbox").fill("name 1");
  await page.getByTestId("card").first().getByRole("textbox").press("Tab");

  // make sure the new card was created.
  await expect(page.getByTestId("card")).toHaveCount(2);
});

test("the next list item is focussed after user presses tab", async ({
  page,
}) => {
  await page.goto("http://localhost:3000");
  await page.waitForLoadState("networkidle"); // Firefox requires a delay

  // add item to list
  await page.getByRole("button", { name: "Add a name" }).click();
  await page.getByTestId("card").nth(0).getByRole("textbox").fill("name 1");

  // add another item to list
  await page.getByRole("button", { name: "Add a name" }).click();
  await page.getByTestId("card").nth(1).getByRole("textbox").fill("name 2");

  await page.getByTestId("card").nth(0).getByRole("textbox").press("Tab");

  // make sure the new card was created.
  await expect(
    page.getByTestId("card").nth(1).getByRole("textbox"),
  ).toBeFocused();
});

test("the next list item is focussed after user presses enter", async ({
  page,
}) => {
  await page.goto("http://localhost:3000");
  await page.waitForLoadState("networkidle"); // Firefox requires a delay

  // add item to list
  await page.getByRole("button", { name: "Add a name" }).click();
  await page.getByTestId("card").nth(0).getByRole("textbox").fill("name 1");

  // add another item to list
  await page.getByRole("button", { name: "Add a name" }).click();
  await page.getByTestId("card").nth(1).getByRole("textbox").fill("name 2");

  await page.getByTestId("card").nth(0).getByRole("textbox").press("Enter");

  // make sure the new card was created.
  await expect(
    page.getByTestId("card").nth(1).getByRole("textbox"),
  ).toBeFocused();
});

test("previous list item is focussed when shift+tab is pressed", async ({
  page,
}) => {
  await page.goto("http://localhost:3000");
  await page.waitForLoadState("networkidle"); // Firefox requires a delay

  // add item to list
  await page.getByRole("button", { name: "Add a name" }).click();
  await page.getByTestId("card").nth(0).getByRole("textbox").fill("Name 1");

  // add another item to list
  await page.getByRole("button", { name: "Add a name" }).click();
  await page.getByTestId("card").nth(1).getByRole("textbox").fill("Name 2");

  // Press shift and tab at the same time
  await page.getByTestId("card").nth(1).getByRole("textbox").press("Shift+Tab");

  // make sure the card is no longer focussed
  await expect(
    page.getByTestId("card").nth(0).getByRole("textbox"),
  ).toBeFocused();
});

test("the spin button is only visible when there are two or more list items", async ({
  page,
}) => {
  await page.goto("http://localhost:3000");
  await page.waitForLoadState("networkidle"); // Firefox requires a delay

  // add item to list
  await page.getByRole("button", { name: "Add a name" }).click();
  await page.getByTestId("card").nth(0).getByRole("textbox").fill("name 1");

  // make sure the spin button is not visible
  await expect(page.getByRole("button", { name: "Spin" })).toBeHidden();

  // add a second item
  await page.getByRole("button", { name: "Add a name" }).click();
  await page.getByTestId("card").nth(1).getByRole("textbox").fill("name 2");

  // make sure the spin button is visible
  await expect(page.getByRole("button", { name: "Spin" })).toBeVisible();
});

test("the spin button is hidden after it's clicked", async ({ page }) => {
  await page.goto("http://localhost:3000");
  await page.waitForLoadState("networkidle"); // Firefox requires a delay

  // add item to list
  await page.getByRole("button", { name: "Add a name" }).click();
  await page.getByTestId("card").nth(0).getByRole("textbox").fill("name 1");

  // add a second item
  await page.getByRole("button", { name: "Add a name" }).click();
  await page.getByTestId("card").nth(1).getByRole("textbox").fill("name 2");

  // click the spin button
  await page.getByRole("button", { name: "Spin" }).click();

  // make sure the spin button is visible
  await expect(page.getByRole("button", { name: "Spin" })).toBeHidden();
});

test("the reset button is visible after the spin button is clicked", async ({
  page,
}) => {
  await page.goto("http://localhost:3000");
  await page.waitForLoadState("networkidle"); // Firefox requires a delay

  // add item to list
  await page.getByRole("button", { name: "Add a name" }).click();
  await page.getByTestId("card").nth(0).getByRole("textbox").fill("name 1");

  // add a second item
  await page.getByRole("button", { name: "Add a name" }).click();
  await page.getByTestId("card").nth(1).getByRole("textbox").fill("name 2");

  // click the spin button
  await page.getByRole("button", { name: "Spin" }).click();

  // make sure the spin button is visible
  await expect(page.getByRole("button", { name: "Reset" })).toBeVisible();
});

test("not picked cards have half opacity after spin is clicked", async ({
  page,
}) => {
  await page.goto("http://localhost:3000");
  await page.waitForLoadState("networkidle"); // Firefox requires a delay

  // add item to list
  await page.getByRole("button", { name: "Add a name" }).click();
  await page.getByTestId("card").nth(0).getByRole("textbox").fill("name 1");

  // add a second item
  await page.getByRole("button", { name: "Add a name" }).click();
  await page.getByTestId("card").nth(1).getByRole("textbox").fill("name 2");

  // click the spin button
  await page.getByRole("button", { name: "Spin" }).click();

  // make sure a card was picked
  await expect(page.getByTestId("picked-card")).toHaveCount(1);
  await expect(page.getByTestId("card")).toHaveCSS("opacity", "0.5");
});

// add a test for making sure the inputs are disapled when spin is clicked.
// expecty this to currently fail as the functionality hasn't been added
// yet.
