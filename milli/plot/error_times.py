#!/usr/bin/env python3

from plotnine.themes.theme_gray import theme_gray
from plotnine.themes.theme import theme
from plotnine.themes.elements import (element_line, element_rect,
                                      element_text, element_blank)
import sys
import pandas as pd
import numpy as np
import plotnine as p9
import re

from plotnine import *
from plotnine.data import *

import warnings

from io import BytesIO
from math import log10

# this is the width of a column in the latex template
LATEX_TEMPLATE_COLUMNWIDTH =  2.6

# the unit of the latex template column width
LATEX_TEMPLATE_COLUMNWDITH_UNIT = 'in'

# this is the width of the plot
PLOT_WIDTH = LATEX_TEMPLATE_COLUMNWIDTH / 2

# this is the size unit
PLOT_SIZE_UNIT = LATEX_TEMPLATE_COLUMNWDITH_UNIT

# this is the ration of the plot
PLOT_ASPECT_RATIO = 16/12 / 1.4

# this is the plot height
PLOT_HEIGHT = PLOT_WIDTH/PLOT_ASPECT_RATIO

# What machine, max cores, sockets, revision
MACHINES = [('skylake4x', 192, 4, '4b3c410')]

class theme_my538(theme_gray):
    def __init__(self, base_size=7, base_family='DejaVu Serif'):
        theme_gray.__init__(self, base_size)
        bgcolor = '#FFFFFF'
        self.add_theme(
            theme(
                # strip_margin=0,
                # strip_margin_x=0,
                # strip_margin_y=0,

                legend_box_margin=0,
                legend_margin=0,
                axis_title=element_text(size=base_size),
                axis_title_y=element_text(size=base_size, margin={'l': 0, 'r': 0}),
                axis_text_x=element_text(size=base_size, margin={'t': 12}),
                axis_text_y=element_text(size=base_size, margin={'r': 30}),
                axis_ticks_length=0,
                axis_ticks=element_line(size=0.5),
                title=element_text(color='#3C3C3C'),
                legend_text=element_text(size=base_size-1),
                legend_background=element_rect(fill='None', color='#000000',
                                               size=0.2, linetype='None'),
                legend_key=element_rect(fill='#FFFFFF', colour=None),
                panel_background=element_rect(fill=bgcolor),
                panel_border=element_line(size=0.2),
                #panel_grid_major=element_line(
                  #color='#E5E5E5', linetype='solid', size=0.5),
                panel_grid_major=element_blank(),
                panel_grid_minor=element_blank(),
                panel_spacing=0.02,
                plot_background=element_rect(
                    fill=bgcolor, color=bgcolor, size=1),
                plot_margin=0.04,
                strip_background=element_rect(fill='#FFFFFF', size=0.2))
            # ,inplace=True
            )

def error_times_plot(df, i):
    aest = aes(x='tool',
          y='time (s)',
          fill='mode'
    )

    p = (
     ggplot(data=df,
         mapping=aest) +
     scale_fill_manual(['#999', 'blue']) +
     geom_col(stat='identity', position='dodge', linetype='solid') +
     theme_my538() +
     theme(legend_position='none',
           legend_title=element_blank(),
           axis_text_x=element_text(angle=70, hjust=1)) +
     xlab('') +
    #  scale_y_log10() +
     ggtitle('function: ' + ('pop' if i == 1 else 'index'))
    )

    # aest = aes(x='count',
    #       y='time',
    #       color='tool',
    #       shape='tool')

    # p = (
    #  ggplot(data=df,
    #       mapping=aest) +
    #  theme_my538() +
    #  coord_cartesian(ylim=(log10(0.1), log10(100)), expand=True) +
    #  theme(legend_position='left', legend_title=element_blank(), legend_direction='vertical', legend_box='horizontal') +
    #  geom_line() + geom_point() +
    #  ylab('time (sec)') + xlab('pushes')
    # )
     
    # p.save(f"error-times-{i}.png",
    #     dpi=300, width=PLOT_WIDTH, height=PLOT_HEIGHT,
    #     units=PLOT_SIZE_UNIT)
    p.save(f"error-times-{i}.pdf",
        dpi=300, width=PLOT_WIDTH, height=PLOT_HEIGHT,
        units=PLOT_SIZE_UNIT)
    # p.save(f"error-times-{i}.pgf",
    #     dpi=300, width=PLOT_WIDTH, height=PLOT_HEIGHT,
    #     units=PLOT_SIZE_UNIT)

if __name__ == '__main__':
    warnings.filterwarnings('ignore')
    pd.set_option('display.max_rows', 500)
    pd.set_option('display.max_columns', 500)
    pd.set_option('display.width', 1000)
    pd.set_option('display.expand_frame_repr', True)

    df = pd.read_csv("linked-list-errors.csv", header=None)
    df = df.rename(columns={df.columns[0]: "mode", df.columns[1]: "case", df.columns[2]: "tool", df.columns[3]: "time (s)"})
    df['tool'] = df['tool'].replace('Fstarlowstar', 'F*')
    df_tools = ['Verus', 'Creusot', 'Dafny', 'F*', 'Prusti']
    # df['index'] = df.index
    tools_cat = pd.Categorical(df['tool'], categories=df_tools)
    df['mode'] = df['mode'].replace('base', 'success')
    print(df)
    df['mode'] = pd.Categorical(df['mode'], categories=['success', 'error'])
    df = df.assign(tool = tools_cat)
    df1 = df[df['case'] == 1]
    df2 = df[df['case'] == 2]
    print(df1)
    print(df2)
    # df_pivot = df.pivot(index=df.columns[0], columns=df.columns[1])
    error_times_plot(df1, 1)
    error_times_plot(df2, 2)

