import { test, expect } from "@playwright/test";

// **********************************************************************
// make sure to re-add the title test once the title issue is resolved
// **********************************************************************
// test("homepage has title", async ({ page }) => {
//   await page.goto("http://localhost:3000");
//   await expect(page).toHaveTitle("Random.Pick");
// });

test("homepage disappears when an item is added to list", async ({ page }) => {
  await page.goto("http://localhost:3000");
  await page.waitForTimeout(1000); // Firefox requires a delay

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
  await page.waitForTimeout(1000); // Firefox requires a delay

  // make sure no cards are visible to the user when the page first opens
  await expect(page.getByTestId("card")).toHaveCount(0);

  // add item to list
  await page.getByRole("button", { name: "Add a name" }).click();

  // make sure one card is added when the button is clicked.
  await expect(page.getByTestId("card")).toHaveCount(1);
});
