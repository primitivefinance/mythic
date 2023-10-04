import matplotlib.pyplot as plt
import seaborn as sns

# Set the style and color palette of Seaborn
sns.set_theme(style="whitegrid")
sns.set_palette("tab10")

def customize_plot(ax, title, xlabel, ylabel):
    ax.set_title(title, fontsize=16)
    ax.set_xlabel(xlabel, fontsize=14)
    ax.set_ylabel(ylabel, fontsize=14)
    ax.legend(fontsize=12)
    ax.grid(True)

def plot_prices(ax, mean_df, std_df):
    sns.lineplot(x=mean_df.index, y='liquid_exchange_prices', data=mean_df, label='Liquid Exchange Prices', ax=ax, linewidth=2, color='blue')
    ax.fill_between(mean_df.index, mean_df['liquid_exchange_prices'] - std_df['liquid_exchange_prices'], mean_df['liquid_exchange_prices'] + std_df['liquid_exchange_prices'], color='blue', alpha=0.2)

    sns.lineplot(x=mean_df.index, y='portfolio_prices', data=mean_df, label='Portfolio Prices', ax=ax, linewidth=2, color='orange')
    ax.fill_between(mean_df.index, mean_df['portfolio_prices'] - std_df['portfolio_prices'], mean_df['portfolio_prices'] + std_df['portfolio_prices'], color='orange', alpha=0.2)
    customize_plot(ax, 'Prices Over Time', 'Index', 'Price')


def plot_arbitrageur_balances_and_values(ax, df):
    # Plot the balances
    sns.lineplot(x=df.index, y='arbitrageur_relative_balances_x', data=df, label='Relative Arbitrageur Balances X', ax=ax, linewidth=2, alpha=0.5, legend=False)
    sns.lineplot(x=df.index, y='arbitrageur_relative_balances_y', data=df, label='Relative Arbitrageur Balances Y', ax=ax, linewidth=2, alpha=0.5, legend=False)

    # Create a second y-axis to plot the portfolio value
    ax2 = ax.twinx()
    sns.lineplot(x=df.index, y='arbitrageur_portfolio_value', data=df, label='Arbitrageur Portfolio Value', ax=ax2, linewidth=2, color='purple', legend=False)

    # Customize the plot
    ax.set_title('Arbitrageur Balances and Portfolio Values Over Time', fontsize=16)
    ax.set_xlabel('Index', fontsize=14)
    ax.set_ylabel('Balance in WAD', fontsize=14)
    ax.grid(True)

    ax2.set_ylabel('Portfolio Value in WAD', fontsize=14)

    # Combine legends from both axes
    lines, labels = ax.get_legend_handles_labels()
    lines2, labels2 = ax2.get_legend_handles_labels()
    ax2.legend(lines + lines2, labels + labels2, loc=0, fontsize=12)

def plot_portfolio_reserves(ax, mean_df, std_df):
    # Plot the balances
    sns.lineplot(x=mean_df.index, y='portfolio_reserves_x', data=mean_df, label='Portfolio Reserves X', ax=ax, linewidth=2, alpha=0.5, legend=False, color='blue')
    ax.fill_between(mean_df.index, mean_df['portfolio_reserves_x'] - std_df['portfolio_reserves_x'], mean_df['portfolio_reserves_x'] + std_df['portfolio_reserves_x'], color='blue', alpha=0.1)
    sns.lineplot(x=mean_df.index, y='portfolio_reserves_y', data=mean_df, label='Portfolio Reserves Y', ax=ax, linewidth=2, alpha=0.5, legend=False, color='orange')
    ax.fill_between(mean_df.index, mean_df['portfolio_reserves_y'] - std_df['portfolio_reserves_y'], mean_df['portfolio_reserves_y'] + std_df['portfolio_reserves_y'], color='orange', alpha=0.1)

    customize_plot(ax, 'Portfolio Reserves Over Time', 'Index', 'Value in WAD')

def plot_lp_portfolio_values(ax, mean_df, std_df):
    sns.lineplot(x=mean_df.index, y='lp_portfolio_value', data=mean_df, label='Portfolio Value', ax=ax, linewidth=2, alpha=1, legend=False, color='purple')
    ax.fill_between(mean_df.index, mean_df['lp_portfolio_value'] - std_df['lp_portfolio_value'], mean_df['lp_portfolio_value'] + std_df['lp_portfolio_value'], color='purple', alpha=0.3)
    customize_plot(ax, 'LP Portfolio Values Over Time', 'Index', 'Value in WAD')

def plot_all(mean_df, std_df, filename):
    fig, axes = plt.subplots(3, 1, figsize=(16, 20))

    plot_prices(axes[0], mean_df, std_df)
    plot_portfolio_reserves(axes[1], mean_df, std_df)
    plot_lp_portfolio_values(axes[2], mean_df, std_df)
    # plot_arbitrageur_balances_and_values(axes[1], df)
    

    plt.tight_layout()  # Adjusts subplot params for better layout
    plt.savefig(filename)

def plot_heatmaps(df):
    # Pivot the DataFrame to get a 2D grid, one for mean and one for std_dev
    mean_pivot = df.pivot(index="fee_basis_points", columns="volatility_basis_points", values="mean_total_fees")
    std_pivot = df.pivot(index="fee_basis_points", columns="volatility_basis_points", values="std_total_fees")


    # Create a figure and a 1x2 grid of subplots
    fig, axes = plt.subplots(1, 2, figsize=(16, 8))

    # Plot the heatmap for mean values
    sns.heatmap(mean_pivot, annot=True, fmt=".2f", cmap="rocket", ax=axes[0])
    axes[0].set_title("Mean Total Fees", fontsize=16)
    axes[0].set_xlabel("Volatility Basis Points", fontsize=14)
    axes[0].set_ylabel("Fee Basis Points", fontsize=14)

    # Plot the heatmap for standard deviation values
    sns.heatmap(std_pivot, annot=True, fmt=".2f", cmap="rocket", ax=axes[1])
    axes[1].set_title("Standard Deviation Total Fees", fontsize=16)
    axes[1].set_xlabel("Volatility Basis Points", fontsize=14)
    axes[1].set_ylabel("Fee Basis Points", fontsize=14)

    plt.tight_layout()
    plt.savefig(f"heatmaps.png")
