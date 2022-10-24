#!/usr/bin/env python3
import sys
import numpy
import matplotlib as mpl
import matplotlib.pyplot as plt
import argparse


# usage: plot.py [-h] [-x XLABEL] [-t TITLE] [-o OUTPUT] file
# 
# positional arguments:
#   file                  space-separated data file; the first column is the parameter;
#                         the next columns are the throughput in msg/s for:
#                           - async-channel::bounded
#                           - flume::bounded
#                           - futures::mpsc
#                           - kanal
#                           - postage::mpsc
#                           - tachyonix
#                           - thingbuf
#                           - tokio::mpsc
# 
# optional arguments:
#   -h, --help            show this help message and exit
#   -x XLABEL, --xlabel XLABEL
#                         label of the x axis
#   -t TITLE, --title TITLE
#                         title of the plot
#   -o OUTPUT, --output OUTPUT
#                         name of the file to which the PNG plot should be saved


mpl.rcParams['axes.prop_cycle'] = mpl.cycler(color=["tab:blue", "tab:orange", "tab:green", "tab:gray", "tab:purple", "tab:red", "tab:brown", "tab:pink"])

def plot(data, x_label, title, output):
    WIDTH = 0.5  # total width of a group of bars
    MULTIPLIER = 1e-6 # convert y units from msg/s to msg/µs

    parameter_labels = [int(param) for param in data[:,0]]
    channel_labels = ['async-channel::bounded', 'flume::bounded', 'futures::mpsc', 'kanal', 'postage::mpsc', 'tachyonix', 'thingbuf', 'tokio::mpsc']

    data = numpy.transpose(data[:, 1:])
    x = numpy.arange(len(parameter_labels))
    n_channels = len(data)

    ax = plt.subplots()[1]
    
    for i, col in enumerate(data):
        delta = -WIDTH/2.0 + i*WIDTH/(n_channels-1)
        ax.bar(x + delta, col*MULTIPLIER, WIDTH/n_channels, label=channel_labels[i])

    if title is not None:
        ax.set_title(title)
    if x_label is not None:
        ax.set_xlabel(x_label)
    ax.set_ylabel('msg/µs')
    ax.set_xticks(x)
    ax.set_xticklabels(parameter_labels)
    ax.legend(loc='upper left')

    if output is None:
        plt.show()
    else:
        plt.savefig(output, format='png', dpi=150, bbox_inches='tight')



if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument("file", help="""
space-separated data file; the first column is the
parameter; the next columns are the throughput in msg/s for:
async-channel::bounded, flume::bounded, futures::mpsc, kanal,
postage::mpsc, tachyonix, thingbuf, tokio::mpsc""")
    parser.add_argument("-x", "--xlabel", help="label of the x axis")
    parser.add_argument("-t", "--title", help="title of the plot")
    parser.add_argument("-o", "--output", help="name of the file to which the PNG plot should be saved")
    args = parser.parse_args()

    with open(args.file) as f:
        data = numpy.loadtxt(f)
        plot(data, args.xlabel, args.title, args.output)

