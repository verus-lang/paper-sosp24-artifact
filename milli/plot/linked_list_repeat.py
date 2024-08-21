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
LATEX_TEMPLATE_COLUMNWIDTH =  2.0

# the unit of the latex template column width
LATEX_TEMPLATE_COLUMNWDITH_UNIT = 'in'

# this is the width of the plot
PLOT_WIDTH = LATEX_TEMPLATE_COLUMNWIDTH

# this is the size unit
PLOT_SIZE_UNIT = LATEX_TEMPLATE_COLUMNWDITH_UNIT

# this is the ration of the plot
PLOT_ASPECT_RATIO = 16/12

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
                panel_border=element_line(size=0.2), # THIS IS THE CULPRIT
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

def memory_plot(df):
    aest = aes(x='count',
          y='time',
          color='tool',
          shape='tool')

    p = (
     ggplot(data=df,
          mapping=aest) +
     theme_my538() +
     scale_y_log10() +
     coord_cartesian(ylim=(log10(0.1), log10(100)), expand=True) +
     theme(legend_position='left', legend_title=element_blank(), legend_direction='vertical', legend_box='horizontal') +
     geom_line() + geom_point() +
     ylab('time (sec)') + xlab('pushes')
    )
     
    p.save("linked-list-memory-reasoning.png",
        dpi=300, width=PLOT_WIDTH, height=PLOT_HEIGHT,
        units=PLOT_SIZE_UNIT)
    p.save("linked-list-memory-reasoning.pdf",
        dpi=300, width=PLOT_WIDTH, height=PLOT_HEIGHT,
        units=PLOT_SIZE_UNIT)

if __name__ == '__main__':
    warnings.filterwarnings('ignore')
    pd.set_option('display.max_rows', 500)
    pd.set_option('display.max_columns', 500)
    pd.set_option('display.width', 1000)
    pd.set_option('display.expand_frame_repr', True)

    df = pd.read_csv("linked-list-repeat.csv", header=None)
    df = df.rename(columns={df.columns[0]: "count", df.columns[1]: "tool", df.columns[2]: "time"})
    df = df[df['time'] != float('inf')]
    df['tool'] = df['tool'].replace('Fstarlowstar', 'F*')
    df_tools = ['F*', 'Prusti', 'Dafny', 'Creusot', 'Verus']
    tools_cat = pd.Categorical(df['tool'], categories=df_tools)
    df = df.assign(tool = tools_cat)
    print(df)
    # df_pivot = df.pivot(index=df.columns[0], columns=df.columns[1])
    memory_plot(df)

